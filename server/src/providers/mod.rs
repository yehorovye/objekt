use crate::structs::metadata::Metadata;

pub mod fs;
#[cfg(feature = "memory")]
pub mod memory;

/// A trait that defines how a cache backend should behave.
///
/// This is generic over the type of value you're caching (`T`),
/// which must implement `Clone`.
pub trait CacheProvider<T: Clone> {
    /// Looks up a value by key.
    ///
    /// Returns `Some(value)` if the key exists, or `None` otherwise.
    async fn entry(&self, key: String) -> Option<T>;

    /// Checks if a given key exists in the cache.
    ///
    /// Returns `true` if the key is present, or `false` if it's not.
    async fn has_key(&self, key: String) -> bool;

    /// Attempts to add a new entry to the cache.
    ///
    /// If the key already exists, this returns `None` and does not overwrite the value.
    /// Otherwise, returns `Some(value)` after inserting it.
    async fn add(&self, key: String, value: T, issuer: String) -> Option<T>;

    /// Removes an entry from the cache
    async fn remove(&self, key: String) -> Option<T>;

    /// Inserts or updates a value in the cache.
    ///
    /// If the key doesn't exist, this behaves like `add`.
    /// If the key does exist, the value is replaced.
    async fn upsert(&self, key: String, value: T, issuer: String) -> T;

    /// Lists all keys and values currently stored in the cache.
    async fn list(&self) -> Vec<(String, T)>;

    /// Retrieves metadata for a given key.
    ///
    /// Returns `Some(metadata)` if the key exists and has metadata.
    /// Returns `None` if the key does not exist.
    async fn metadata(&self, key: String) -> Option<Metadata>;
}
