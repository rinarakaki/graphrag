//! Parameterization settings for the default configuration.

use crate::config::enums::CacheType;

/// The default configuration section for Cache.
pub struct CacheConfig {
    /// The cache type to use.
    pub r#type: CacheType,

    /// The base directory for the cache.
    pub base_dir: String,

    /// The cache connection string to use.
    pub connection_string: Option<String>,

    /// The cache container name to use.
    pub container_name: Option<String>,

    /// The storage account blob url to use.
    pub storage_account_blob_url: Option<String>,

    /// The cosmosdb account url to use.
    pub cosmosdb_account_url: Option<String>,
}

impl Default for CacheConfig {
    /// Default values for cache.
    fn default() -> Self {
        CacheConfig {
            r#type: CacheType::File,
            base_dir: "cache".into(),
            connection_string: None,
            container_name: None,
            storage_account_blob_url: None,
            cosmosdb_account_url: None,
        }
    }
}
