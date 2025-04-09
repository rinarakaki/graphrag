//! Base classes for vector stores.

use crate::data_model::types::TextEmbedder;

const DEFAULT_VECTOR_SIZE: usize = 1536;

/// A document that is stored in vector storage.
pub struct VectorStoreDocument {
    /// unique id for the document
    pub id: str | int,

    pub text: Option<String>,
    pub vector: Option<Vec<f64>>,

    /// store any additional metadata, e.g. title, date ranges, etc
    pub attributes: HashMap<String, Any>, // = field(default_factory=dict),
}

/// A vector storage search result.
pub struct VectorStoreSearchResult {
    /// Document that was found.
    document: VectorStoreDocument,

    /// Similarity score between -1 and 1. Higher is more similar.
    score: f64,
}

/// The base class for vector storage data-access classes.
pub trait BaseVectorStore {
    // fn __init__(
    //     self,
    //     collection_name: str,
    //     db_connection: Any | None = None,
    //     document_collection: Any | None = None,
    //     query_filter: Any | None = None,
    //     **kwargs: Any,
    // ):
    //     self.collection_name = collection_name
    //     self.db_connection = db_connection
    //     self.document_collection = document_collection
    //     self.query_filter = query_filter
    //     self.kwargs = kwargs

    /// Connect to vector storage.
    fn connect(
        &mut self,
        // **kwargs: Any
    );

    /// Load documents into the vector-store.
    fn load_documents(
        &self,
        documents: Vec<VectorStoreDocument>,
        overwrite: bool, // = True
    );

    /// Perform ANN search by vector.
    fn similarity_search_by_vector(
        &self,
        query_embedding: Vec<f64>,
        k: usize, // = 10,
        // **kwargs: Any
    ) -> Vec<VectorStoreSearchResult>;

    /// Perform ANN search by text.
    fn similarity_search_by_text(
        &self,
        text: &str,
        text_embedder: TextEmbedder,
        k: usize, // = 10,
        // **kwargs: Any
    ) -> Vec<VectorStoreSearchResult>;

    /// Build a query filter to filter documents by id.
    fn filter_by_id(
        &self,
        include_ids: Vec<String> | Vec<int>
    ) -> Any;

    /// Search for a document by id.
    fn search_by_id(&self, id: &str) -> VectorStoreDocument;
}
