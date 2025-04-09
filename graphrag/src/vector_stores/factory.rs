//! A package containing a factory and supported vector store types.

// use crate::vector_stores::azure_ai_search::AzureAISearchVectorStore;
// use crate::vector_stores::base::BaseVectorStore;
// use crate::vector_stores::cosmosdb::CosmosDBVectorStore;
use crate::vector_stores::lancedb::LanceDBVectorStore;

/// The supported vector store types.
pub enum VectorStoreType {
    LanceDB,
    AzureAISearch,
    CosmosDB,
}

impl VectorStoreType {
    pub fn as_str(&self) -> &str {
        match self {
            VectorStoreType::LanceDB => "lancedb",
            VectorStoreType::AzureAISearch => "azure_ai_search",
            VectorStoreType::CosmosDB => "cosmosdb",
        }
    }
}

/**
A factory for vector stores.

Includes a method for users to register a custom vector store implementation.
*/
pub struct VectorStoreFactory {
    vector_store_types: ClassVar[dict[str, type]] = {}
}

impl VectorStoreFactory {
    /// Register a custom vector store implementation.
    @classmethod
    pub fn register(cls, vector_store_type: str, vector_store: type) {
        cls.vector_store_types[vector_store_type] = vector_store
    }

    /// Create or get a vector store from the provided type.
    pub fn create_vector_store(
        vector_store_type: VectorStoreType | str,
        kwargs: dict,
    ) -> BaseVectorStore {
        match vector_store_type:
            VectorStoreType::LanceDB => LanceDBVectorStore(**kwargs),
            VectorStoreType::AzureAISearch => AzureAISearchVectorStore(**kwargs),
            VectorStoreType::CosmosDB => CosmosDBVectorStore(**kwargs),
    }
}
