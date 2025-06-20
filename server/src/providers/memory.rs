use std::sync::Arc;

use crate::structs::metadata::Metadata;
use chrono::Utc;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serde_json;

use super::CacheProvider;

/// A thread-safe, in-memory cache implementation using `DashMap`.
///
/// Stores key-value pairs in memory and keeps metadata (e.g. creation time, version, issuer)
/// alongside each value.
pub struct MemoryProvider<T: Clone + Serialize + for<'a> Deserialize<'a>> {
    storage: Arc<DashMap<String, T>>,
    meta: Arc<DashMap<String, String>>,
}

impl<T: Clone + Serialize + for<'a> Deserialize<'a>> MemoryProvider<T> {
    /// Creates a new memory cache with a given initial capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            storage: Arc::new(DashMap::with_capacity(capacity)),
            meta: Arc::new(DashMap::with_capacity(capacity)),
        }
    }

    /// Internal helper to serialize and store metadata.
    fn write_metadata(&self, key: &str, metadata: &Metadata) {
        if let Ok(json) = serde_json::to_string(metadata) {
            self.meta.insert(Self::meta_key(key), json);
        }
    }

    /// Internal helper to format metadata keys.
    fn meta_key(key: &str) -> String {
        format!("{key}$")
    }
}

impl<T: Clone + Serialize + for<'a> Deserialize<'a>> CacheProvider<T> for MemoryProvider<T> {
    async fn entry(&self, key: String) -> Option<T> {
        self.storage.get(&key).map(|entry| entry.value().clone())
    }

    async fn add(&self, key: String, value: T, issuer: String) -> Option<T> {
        if self.storage.contains_key(&key) {
            return None;
        }

        let metadata = Metadata {
            created_at: Utc::now().to_rfc3339(),
            version: 0,
            issuer,
        };

        self.write_metadata(&key, &metadata);
        self.storage.insert(key.clone(), value.clone());

        Some(value)
    }

    async fn update(&self, key: String, value: T, issuer: String) -> Option<T> {
        if !self.storage.contains_key(&key) {
            return None;
        }

        if let Some(mut meta_entry) = self.meta.get_mut(&Self::meta_key(&key)) {
            if let Ok(mut metadata) = serde_json::from_str::<Metadata>(&meta_entry) {
                metadata.version += 1;
                metadata.issuer = issuer;
                if let Ok(updated) = serde_json::to_string(&metadata) {
                    *meta_entry = updated;
                }
            }
        }

        self.storage.insert(key.clone(), value.clone());
        Some(value)
    }

    async fn metadata(&self, key: String) -> Option<Metadata> {
        self.meta
            .get(&Self::meta_key(&key))
            .and_then(|entry| serde_json::from_str(entry.value()).ok())
    }

    async fn remove(&self, key: String) -> Option<T> {
        self.storage.remove(&key).map(|(_, value)| value)
    }

    async fn list(&self) -> Vec<(String, T)> {
        self.storage
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect()
    }

    async fn purge(&self, issuer: String) {
        let keys_to_remove: Vec<String> = self
            .meta
            .iter()
            .filter_map(|entry| {
                let meta_json = entry.value();
                if let Ok(meta) = serde_json::from_str::<Metadata>(meta_json) {
                    if meta.issuer == issuer {
                        return Some(entry.key().trim_end_matches('$').to_owned());
                    }
                }
                None
            })
            .collect();

        for key in keys_to_remove {
            self.storage.remove(&key);
            self.meta.remove(&Self::meta_key(&key));
        }
    }
}
