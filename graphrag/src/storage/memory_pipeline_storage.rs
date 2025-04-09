//! A module containing 'InMemoryStorage' model.

use crate::storage::file_pipeline_storage::FilePipelineStorage;
use crate::storage::pipeline_storage::PipelineStorage;

/// In memory storage class definition.
pub struct MemoryPipelineStorage(FilePipelineStorage) {
    _storage: HashMap<String, Box<dyn Any>>,
}

impl MemoryPipelineStorage {
    def __init__(self):
        /// Init method definition.
        super().__init__()
        self._storage = {}

    async def get(
        self, key: str, as_bytes: bool | None = None, encoding: Option<String> /* = None */
    ) -> Any:
        """Get the value for the given key.

        Args:
            - key - The key to get the value for.
            - as_bytes - Whether or not to return the value as bytes.

        Returns
        -------
            - output - The value for the given key.
        """
        return self._storage.get(key)

    async def set(self, key: str, value: Any, encoding: Option<String> /* = None */) -> None:
        """Set the value for the given key.

        Args:
            - key - The key to set the value for.
            - value - The value to set.
        """
        self._storage[key] = value

    async def has(self, key: str) -> bool:
        """Return True if the given key exists in the storage.

        Args:
            - key - The key to check for.

        Returns
        -------
            - output - True if the key exists in the storage, False otherwise.
        """
        return key in self._storage

    async def delete(self, key: str) -> None:
        """Delete the given key from the storage.

        Args:
            - key - The key to delete.
        """
        del self._storage[key]

    async def clear(self) -> None:
        /// Clear the storage.
        self._storage.clear()

    def child(self, name: Option<String>) -> "PipelineStorage":
        /// Create a child storage instance.
        return MemoryPipelineStorage()

    def keys(self) -> Vec<str>:
        /// Return the keys in the storage.
        return list(self._storage.keys())
}
