use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tokio::{fs, io::AsyncWriteExt};

use crate::structs::metadata::Metadata;

use super::CacheProvider;

/// A simple file‑based cache.
///
/// * Each value is stored as `<key>` (JSON).
/// * Its metadata lives next to it as `<key>.meta`.
/// * All files reside in a single directory (`path`).
pub struct FileSystemProvider<T: Clone> {
    path: PathBuf,
    _marker: PhantomData<T>, // uh.
}

impl<T: Clone> FileSystemProvider<T> {
    /// Create the cache directory if it does not exist and return a provider.
    pub async fn new(path: PathBuf) -> Result<Self> {
        if !path.exists() {
            fs::create_dir_all(&path).await?;
        }

        Ok(Self {
            path,
            _marker: PhantomData,
        })
    }

    fn value_path(&self, key: &str) -> PathBuf {
        self.path.join(key)
    }

    fn meta_path(&self, key: &str) -> PathBuf {
        self.path.join(format!("{key}.meta"))
    }

    async fn write_json<P: AsRef<Path>, V: Serialize>(&self, path: P, value: &V) -> Result<()> {
        let json = serde_json::to_string(value)?;
        let mut f = fs::File::create(path).await?;
        f.write_all(json.as_bytes()).await?;
        Ok(())
    }

    async fn read_json<P, V>(&self, path: P) -> Option<V>
    where
        P: AsRef<Path>,
        V: DeserializeOwned,
    {
        let data = fs::read_to_string(path).await.ok()?;
        serde_json::from_str(&data).ok()
    }
}

impl<T: Clone + Serialize + for<'a> Deserialize<'a>> CacheProvider<T> for FileSystemProvider<T> {
    async fn entry(&self, key: String) -> Option<T> {
        self.read_json(self.value_path(&key)).await
    }

    async fn add(&self, key: String, value: T, issuer: String) -> Option<T> {
        let value_path = self.value_path(&key);
        if value_path.exists() {
            return None;
        }

        if self.write_json(&value_path, &value).await.is_err() {
            return None;
        }

        let metadata = Metadata {
            created_at: Utc::now().to_rfc3339(),
            version: 0,
            issuer,
        };
        let _ = self.write_json(self.meta_path(&key), &metadata).await;

        Some(value)
    }

    async fn update(&self, key: String, value: T, issuer: String) -> Option<T> {
        let value_path = self.value_path(&key);
        if !value_path.exists() {
            return None;
        }

        if self.write_json(&value_path, &value).await.is_err() {
            return None;
        }

        let meta_path = self.meta_path(&key);
        let mut meta = self
            .read_json::<_, Metadata>(&meta_path)
            .await
            .unwrap_or(Metadata {
                created_at: String::new(),
                version: 0,
                issuer: issuer.clone(),
            });

        meta.version += 1;
        meta.created_at = Utc::now().to_rfc3339();
        meta.issuer = issuer;

        let _ = self.write_json(&meta_path, &meta).await;
        Some(value)
    }

    async fn metadata(&self, key: String) -> Option<Metadata> {
        self.read_json(self.meta_path(&key)).await
    }

    async fn remove(&self, key: String) -> Option<T> {
        let value_path = self.value_path(&key);
        let meta_path = self.meta_path(&key);

        let existing = self.read_json::<_, T>(&value_path).await;
        let _ = fs::remove_file(value_path).await;
        let _ = fs::remove_file(meta_path).await;

        existing
    }

    async fn list(&self) -> Vec<(String, T)> {
        let mut out = Vec::new();
        let mut entries = match fs::read_dir(&self.path).await {
            Ok(e) => e,
            Err(_) => return out,
        };

        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|e| e == "meta")
            {
                continue;
            }

            if let (Some(filename), Some(value)) = (
                path.file_name().and_then(|n| n.to_str()).map(str::to_owned),
                self.read_json::<_, T>(&path).await,
            ) {
                out.push((filename, value));
            }
        }
        out
    }

    async fn purge(&self, issuer: String) {
        if let Ok(mut entries) = fs::read_dir(&self.path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();

                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if ext != "meta" {
                        continue;
                    }

                    if let Some(meta) = self.read_json::<_, Metadata>(&path).await {
                        if meta.issuer == issuer {
                            if let Some(filename) = path.file_stem().and_then(|n| n.to_str()) {
                                let value_path = self.value_path(filename);
                                let _ = fs::remove_file(&value_path).await;
                                let _ = fs::remove_file(&path).await;
                            }
                        }
                    }
                }
            }
        }
    }
}
