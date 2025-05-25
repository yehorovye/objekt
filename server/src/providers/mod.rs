use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod fs;

/// Stores optional metadata for cache entries.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// Timestamp for when the entry was created.
    created_at: String,
    /// Timestamp for when the entry was last updated.
    updated_at: String,
    /// Optional comment or note attached to the entry.
    comment: Option<String>,
    /// Version of the cache server
    version: u8,
    /// Owner of the cache value
    issuer: String,
}

/// todo: I am aware that this trait should have generics for better management and scalability
/// but rust be kinda bitch sometimes
///
/// A trait that defines how a cache backend should behave.
///
/// This is generic over the type of value you're caching (`T`),
/// which must implement `Clone`.
pub trait CacheProvider {
    /// Looks up a value by key.
    ///
    /// Returns `Some(value)` if the key exists, or `None` otherwise.
    async fn entry(&self, key: String) -> Option<Value>;

    /// Checks if a given key exists in the cache.
    ///
    /// Returns `true` if the key is present, or `false` if it's not.
    async fn has_key(&self, key: String) -> bool;

    /// Attempts to add a new entry to the cache.
    ///
    /// If the key already exists, this returns `None` and does not overwrite the value.
    /// Otherwise, returns `Some(value)` after inserting it.
    async fn add(&self, key: String, value: Value, issuer: String) -> Option<Value>;

    /// Inserts or updates a value in the cache.
    ///
    /// If the key doesn't exist, this behaves like `add`.
    /// If the key does exist, the value is replaced.
    async fn upsert(&self, key: String, value: Value, issuer: String) -> Value;

    /// Lists all keys and values currently stored in the cache.
    async fn list(&self) -> Vec<(String, Value)>;

    /// Associates metadata with an existing key.
    ///
    /// Returns `Some(metadata)` if the key exists and metadata was set.
    /// Returns `None` if the key does not exist.
    async fn set_metadata(&self, key: String, metadata: Metadata) -> Option<Metadata>;

    /// Retrieves metadata for a given key.
    ///
    /// Returns `Some(metadata)` if the key exists and has metadata.
    /// Returns `None` if the key does not exist.
    async fn metadata(&self, key: String) -> Option<Metadata>;
}
