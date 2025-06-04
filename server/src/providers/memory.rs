use std::sync::Arc;

use crate::structs::metadata::Metadata;
use chrono::Utc;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serde_json;

use super::CacheProvider;

pub struct MemoryProvider<T: Clone + Serialize + for<'a> Deserialize<'a>> {
    storage: Arc<DashMap<String, T>>,
    meta: Arc<DashMap<String, String>>,
}

impl<T: Clone + Serialize + for<'a> Deserialize<'a>> MemoryProvider<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            storage: Arc::new(DashMap::with_capacity(capacity)),
            meta: Arc::new(DashMap::with_capacity(capacity)),
        }
    }
}

impl<T: Clone + Serialize + for<'a> Deserialize<'a>> CacheProvider<T> for MemoryProvider<T> {
    async fn entry(&self, key: String) -> Option<T> {
        self.storage.get(&key).map(|v| v.value().clone())
    }

    async fn add(&self, key: String, value: T, issuer: String) -> Option<T> {
        let exists = self.storage.contains_key(&key);

        match exists {
            false => {
                let metadata = Metadata {
                    created_at: Utc::now().to_string(),
                    version: 0,
                    issuer,
                };

                // Insert metadata
                if let Ok(meta) = serde_json::to_string(&metadata) {
                    self.meta.insert(format!("{key}$"), meta);
                }

                // Insert the actual value
                self.storage.insert(key, value)
            }
            true => None,
        }
    }

    async fn metadata(&self, key: String) -> Option<Metadata> {
        self.meta
            .get(&format!("{key}$"))
            .and_then(|v| serde_json::from_str(v.value()).ok())
    }

    async fn upsert(&self, key: String, value: T, issuer: String) -> T {
        let exists = self.storage.contains_key(&key);

        match exists {
            false => self.add(key, value, issuer).await.unwrap(), // we unwrap cuz the value
            // doesn't exists and should give no error
            true => self.storage.insert(key, value).unwrap(),
        }
    }

    async fn has_key(&self, key: String) -> bool {
        self.storage.contains_key(&key)
    }

    async fn remove(&self, key: String) -> Option<T> {
        self.storage.remove(&key).map(|v| v.1)
    }

    async fn list(&self) -> Vec<(String, T)> {
        self.storage
            .iter()
            .map(|e| (e.key().to_owned(), e.value().to_owned()))
            .collect()
    }
}
