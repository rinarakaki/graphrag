//! A module containing 'PipelineStorage' model.

use std::any::Any;
use std::collections::HashMap;

use crate::logger::base::ProgressLogger;

/// Provide a storage interface for the pipeline. This is where the pipeline will store its output data.
pub trait PipelineStorage<T> {
    /// Find files in the storage using a file pattern, as well as a custom filter function.
    fn find(
        &self,
        file_pattern: String, //re::Pattern[String],
        base_dir: Option<String>,
        progress: Option<impl ProgressLogger>,
        file_filter: Option<HashMap<String, Box<dyn Any>>>,
        max_count: usize, // = -1,
    ) -> impl Iterator<Item = (String, HashMap<String, Box<dyn Any>>)>;

    /**
    Get the value for the given key.

    Args:
        - key - The key to get the value for.
        - as_bytes - Whether or not to return the value as bytes.

    Returns
    -------
        - output - The value for the given key.
    */
    async fn get(&self, key: String, as_bytes: Option<bool>, encoding: Option<String>) -> T;

    /**
    Set the value for the given key.

    Args:
        - key - The key to set the value for.
        - value - The value to set.
    */
    async fn set(&mut self, key: &str, value: T, encoding: Option<String>);

    /**
    Return True if the given key exists in the storage.

    Args:
        - key - The key to check for.

    Returns
    -------
        - output - True if the key exists in the storage, False otherwise.
    */
    async fn has(&self, key: &str) -> bool;

    /**
    Delete the given key from the storage.

    Args:
        - key - The key to delete.
    */
    async fn delete(&mut self, key: &str);

    /// Clear the storage.
    async fn clear(&mut self);

    /// Create a child storage instance.
    fn child(&self, name: Option<String>) -> impl PipelineStorage<T>;

    /// List all keys in the storage.
    fn keys(&self) -> Vec<String>;

    /**
    Get the creation date for the given key.

    Args:
        - key - The key to get the creation date for.

    Returns
    -------
        - output - The creation date for the given key.
    */
    async fn get_creation_date(&self, key: String) -> String;
}
