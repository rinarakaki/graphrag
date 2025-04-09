//! A module containing 'PipelineCache' model.

use std::collections::HashMap;

/// Provide a cache interface for the pipeline.
pub trait PipelineCache<T> {
    /**
    Get the value for the given key.

    Args:
        - key - The key to get the value for.
        - as_bytes - Whether or not to return the value as bytes.

    Returns
    -------
        - output - The value for the given key.
    */
    async fn get(&self, key: str) -> T;

    /**
    Set the value for the given key.

    Args:
        - key - The key to set the value for.
        - value - The value to set.
    */
    async fn set(&mut self, key: str, value: T, debug_data: Option<HashMap<String, String>>);

    /**
    Return True if the given key exists in the cache.

    Args:
        - key - The key to check for.

    Returns
    -------
        - output - True if the key exists in the cache, False otherwise.
    */
    async fn has(&self, key: str) -> bool;

    /**
    Delete the given key from the cache.

    Args:
        - key - The key to delete.
    */
    async fn delete(&mut self, key: str);

    /// Clear the cache.
    async fn clear(&mut self);

    /**
    Create a child cache with the given name.

    Args:
    - name - The name to create the sub cache with.
    */
    fn child(&self, name: str) -> impl PipelineCache<T>;
}
