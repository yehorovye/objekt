use std::path::PathBuf;

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::structs::metadata::Metadata;

use super::CacheProvider;

pub struct FileSystemProvider<T: Clone> {
    path: PathBuf,
    _marker: Option<T>, // todo: get rid of ts
}

impl<T: Clone> FileSystemProvider<T> {
    pub async fn new(path: PathBuf) -> Result<Self> {
        if !path.exists() {
            fs::create_dir_all(&path).await?
        }

        Ok(Self {
            path,
            _marker: None,
        })
    }
}

impl<T: Clone + Serialize + for<'a> Deserialize<'a>> CacheProvider<T> for FileSystemProvider<T> {
    async fn entry(&self, key: String) -> Option<T> {
        let file = self.path.join(&key);

        match file.exists() {
            true => {
                if let Ok(value) = fs::read_to_string(file).await {
                    match serde_json::from_str(&value) {
                        Ok(val) => return Some(val),
                        Err(_) => return None,
                    };
                }

                None
            }
            false => None,
        }
    }

    async fn add(&self, key: String, value: T, issuer: String) -> Option<T> {
        let file = self.path.join(&key);

        match file.exists() {
            true => None,
            false => {
                if let Ok(mut value_file) = File::create(&file).await
                    && let Ok(json_value) = serde_json::to_string(&value)
                {
                    let _ = value_file.write_all(json_value.as_bytes()).await;

                    // Write to metadata file
                    let meta_path = self.path.join(format!("{key}.meta"));
                    if let Ok(mut metadata_file) = File::create(meta_path).await
                        && let Ok(parsed_meta) = serde_json::to_string(&Metadata {
                            created_at: Utc::now().to_string(),
                            version: 0,
                            issuer,
                        })
                    {
                        let _ = metadata_file.write_all(parsed_meta.as_bytes()).await;
                    }

                    return Some(value);
                }

                None
            }
        }
    }

    async fn metadata(&self, key: String) -> Option<Metadata> {
        let meta_path = self.path.join(format!("{key}.meta"));

        match fs::read_to_string(&meta_path).await {
            Ok(contents) => serde_json::from_str(&contents).ok(),
            Err(_) => None,
        }
    }

    async fn upsert(&self, key: String, value: T, issuer: String) -> T {
        let file = self.path.join(&key);

        if let Ok(mut value_file) = File::create(&file).await
            && let Ok(json_value) = serde_json::to_string(&value)
        {
            let _ = value_file.write_all(json_value.as_bytes()).await;
        }

        // Update metadata
        let meta_path = self.path.join(format!("{key}.meta"));
        let now = Utc::now().to_string();
        let metadata = Metadata {
            created_at: now.clone(),
            version: 0,
            issuer,
        };

        if let Ok(mut meta_file) = File::create(meta_path).await
            && let Ok(json_metadata) = serde_json::to_string(&metadata)
        {
            let _ = meta_file.write_all(json_metadata.as_bytes()).await;
        }

        value
    }

    async fn has_key(&self, key: String) -> bool {
        self.path.join(&key).exists()
    }

    async fn remove(&self, key: String) -> Option<T> {
        let file = self.path.join(&key);

        if !file.exists() {
            return None;
        }

        let meta_path = self.path.join(format!("{key}.meta"));

        if meta_path.exists() {
            fs::remove_file(meta_path).await.ok();
        }

        fs::remove_file(file).await.ok();

        None
    }

    async fn list(&self) -> Vec<(String, T)> {
        let mut results = Vec::new();

        if let Ok(mut entries) = fs::read_dir(&self.path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();

                if path.is_file()
                    && let Some(filename) = path.file_name().and_then(|n| n.to_str())
                    && !filename.ends_with(".meta")
                    && let Ok(contents) = fs::read_to_string(&path).await
                    && let Ok(value) = serde_json::from_str(&contents)
                {
                    results.push((filename.to_string(), value));
                }
            }
        }

        results
    }
}
