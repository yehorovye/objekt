use serde::{Deserialize, Serialize};

/// Stores optional metadata for cache entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// Timestamp for when the entry was created.
    pub created_at: String,
    /// Version of the cache server
    pub version: u8,
    /// Owner of the cache value
    pub issuer: String,
}
