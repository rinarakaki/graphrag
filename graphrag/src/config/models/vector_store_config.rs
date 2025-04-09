//! Parameterization settings for the default configuration.

use crate::vector_stores::factory::VectorStoreType;

/// The default configuration section for Vector Store.
pub struct VectorStoreConfig {
    /// The vector store type to use.
    pub r#type: String,

    /// The database URI to use.
    pub db_uri: Option<String>,

    /// The database URL when type == azure_ai_search.
    pub url: Option<String>,

    /// The database API key when type == azure_ai_search.
    pub api_key: Option<String>,

    /// The database audience when type == azure_ai_search.
    pub audience: Option<String>,

    /// The container name to use.
    pub container_name: String,

    /// The database name to use when type == cosmos_db.
    pub database_name: Option<String>,

    /// Overwrite the existing data.
    pub overwrite: bool,
}

impl Default for VectorStoreConfig {
    /// Default values for vector stores.
    fn default() -> Self {
        VectorStoreConfig {
            r#type: VectorStoreType::LanceDB.as_str().into(),
            db_uri: Some("output/lancedb".into()),
            url: None,
            api_key: None,
            audience: None,
            container_name: "default".into(),
            database_name: None,
            overwrite: true,
        }
    }
}

// impl VectorStoreConfig {
//     /// Validate the database URI.
//     fn _validate_db_uri(&self) {
//         if self.r#type == VectorStoreType.LanceDB.value && (
//             self.db_uri.is_none() or self.db_uri.strip() == ""
//         ):
//             self.db_uri = vector_store_defaults.db_uri

//         if self.type != VectorStoreType.LanceDB.value and (
//             self.db_uri is not None and self.db_uri.strip() != ""
//         ):
//             msg = "vector_store.db_uri is only used when vector_store.type == lancedb. Please rerun `graphrag init` and select the correct vector store type."
//             raise ValueError(msg)
//     }

//     /// Validate the database URL.
//     fn _validate_url(&self) {
//         if self.r#type == VectorStoreType.AzureAISearch && (
//             self.url.is_none() or self.url.strip() == ""
//         ):
//             msg = "vector_store.url is required when vector_store.type == azure_ai_search. Please rerun `graphrag init` and select the correct vector store type."
//             raise ValueError(msg)

//         if self.type == VectorStoreType.CosmosDB and (
//             self.url.is_none() or self.url.strip() == ""
//         ):
//             msg = "vector_store.url is required when vector_store.type == cosmos_db. Please rerun `graphrag init` and select the correct vector store type."
//             raise ValueError(msg)

//         if self.type == VectorStoreType.LanceDB and (
//             self.url is not None and self.url.strip() != ""
//         ):
//             msg = "vector_store.url is only used when vector_store.type == azure_ai_search or vector_store.type == cosmos_db. Please rerun `graphrag init` and select the correct vector store type."
//             raise ValueError(msg)
//     }

//     @model_validator(mode="after")
//     fn _validate_model(self):
//         /// Validate the model.
//         self._validate_db_uri()
//         self._validate_url()
//         return self
// }
