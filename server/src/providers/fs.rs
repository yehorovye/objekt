use std::path::PathBuf;

use anyhow::Result;
use chrono::Utc;
use serde_json::{self, Value};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use super::{CacheProvider, Metadata};

pub struct FileSystemProvider {
    path: PathBuf,
}

impl FileSystemProvider {
    pub async fn new(path: PathBuf) -> Result<Self> {
        if !path.exists() {
            fs::create_dir_all(&path).await?
        }

        Ok(Self { path })
    }
}

impl CacheProvider for FileSystemProvider {
    async fn entry(&self, key: String) -> Option<Value> {
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

    async fn add(&self, key: String, value: Value, issuer: String) -> Option<Value> {
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
                            comment: Some(String::from("test")),
                            updated_at: Utc::now().to_string(),
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

    async fn upsert(&self, key: String, value: Value, issuer: String) -> Value {
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
            created_at: now.clone(), // todo: ignore
            updated_at: now,
            comment: Some("Updated via upsert".to_string()),
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

    async fn set_metadata(
        &self,
        key: String,
        metadata: super::Metadata,
    ) -> Option<super::Metadata> {
        let file = self.path.join(&key);
        if !file.exists() {
            return None;
        }

        let meta_path = self.path.join(format!("{key}.meta"));
        if let Ok(mut meta_file) = File::create(meta_path).await
            && let Ok(json_metadata) = serde_json::to_string(&metadata)
        {
            let _ = meta_file.write_all(json_metadata.as_bytes()).await;
            return Some(metadata);
        }

        None
    }

    async fn list(&self) -> Vec<(String, Value)> {
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
