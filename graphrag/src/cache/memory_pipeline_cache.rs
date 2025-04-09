//! A module containing 'InMemoryCache' model.

use std::collections::HashMap;

use crate::cache::pipeline_cache::PipelineCache;

/// In memory cache class definition.
pub struct InMemoryCache {
    _cache: HashMap<String, Any>,
    _name: String,
}

impl InMemoryCache {
    /// Init method definition.
    pub fn new(name: Option<String>) -> Self {
        self._cache = {}
        self._name = name or ""
    }
}

impl PipelineCache for InMemoryCache {
    /**
    Get the value for the given key.

    Args:
        - key - The key to get the value for.
        - as_bytes - Whether or not to return the value as bytes.

    Returns
    -------
        - output - The value for the given key.
     */
    async fn get(self, key: str) -> Any {

        key = self._create_cache_key(key)
        return self._cache.get(key)
    }

    /// Set the value for the given key.
    ///
    /// Args:
    /// - key - The key to set the value for.
    /// - value - The value to set.
    async fn set(self, key: str, value: Any, debug_data: Option<HashMap>, /* = None */) -> None {
        key = self._create_cache_key(key)
        self._cache[key] = value
    }

    /// Return True if the given key exists in the storage.
    ///
    /// Args:
    /// - key - The key to check for.
    ///
    /// Returns
    /// -------
    /// - output - True if the key exists in the storage, False otherwise.
    async fn has(self, key: str) -> bool {
        key = self._create_cache_key(key);
        return key in self._cache
    }

    /// Delete the given key from the storage.
    ///
    /// Args:
    ///     - key - The key to delete.
    async fn delete(self, key: str) -> None {
        key = self._create_cache_key(key);
        del self._cache[key]
    }

    /// Clear the storage.
    async fn clear(self) -> None {
        self._cache.clear()
    }

    /// Create a sub cache with the given name.
    fn child(self, name: str) -> PipelineCache {
        InMemoryCache::new(name)
    }

    /// Create a cache key for the given key.
    fn _create_cache_key(self, key: str) -> String {
        foramt!("{self._name}{key}")
    }
}
