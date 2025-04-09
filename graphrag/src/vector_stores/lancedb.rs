//! The LanceDB vector storage implementation package.

//::pyarrow as pa

use crate::data_model::types::TextEmbedder;
use crate::vector_stores::base::{
    BaseVectorStore,
    VectorStoreDocument,
    VectorStoreSearchResult,
};
//::lancedb

/// LanceDB vector storage implementation.
pub struct LanceDBVectorStore;

impl BaseVectorStore for LanceDBVectorStore {
    /// Connect to the vector storage.
    fn connect(&mut self, **kwargs: Any) -> Any {
        self.db_connection = lancedb.connect(kwargs["db_uri"]);
        if (
            self.collection_name
            and self.collection_name in self.db_connection.table_names()
        ) {
            self.document_collection = self.db_connection.open_table(
                self.collection_name
            )
        }
    }

    fn load_documents(
        self, documents: Vec<VectorStoreDocument>, overwrite: bool = True
    ) {
        /// Load documents into vector storage.
        data = [
            {
                "id": document.id,
                "text": document.text,
                "vector": document.vector,
                "attributes": json.dumps(document.attributes),
            }
            for document in documents
            if document.vector is not None
        ]

        if len(data) == 0:
            data = None

        schema = pa.schema([
            pa.field("id", pa.string()),
            pa.field("text", pa.string()),
            pa.field("vector", pa.list_(pa.float64())),
            pa.field("attributes", pa.string()),
        ])
        // NOTE: If modifying the next section of code, ensure that the schema remains the same.
        //       The pyarrow format of the 'vector' field may change if the order of operations is changed
        //       and will break vector search.
        if overwrite:
            if data:
                self.document_collection = self.db_connection.create_table(
                    self.collection_name, data=data, mode="overwrite"
                )
            else:
                self.document_collection = self.db_connection.create_table(
                    self.collection_name, schema=schema, mode="overwrite"
                )
        else:
            # add data to existing table
            self.document_collection = self.db_connection.open_table(
                self.collection_name
            )
            if data:
                self.document_collection.add(data)
    }

    fn filter_by_id(self, include_ids: Vec<String> | Vec<int>) -> Any:
        /// Build a query filter to filter documents by id.
        if len(include_ids) == 0:
            self.query_filter = None
        else:
            if isinstance(include_ids[0], str):
                id_filter = ", ".join([format!("'{id}'") for id in include_ids])
                self.query_filter = format!("id in ({id_filter})")
            else:
                self.query_filter = (
                    format!("id in ({', '.join([str(id) for id in include_ids])})")
                )
        return self.query_filter

    fn similarity_search_by_vector(
        self, query_embedding: Vec<float>, k: int = 10, **kwargs: Any
    ) -> Vec<VectorStoreSearchResult>:
        /// Perform a vector-based similarity search.
        if self.query_filter:
            docs = (
                self.document_collection.search(
                    query=query_embedding, vector_column_name="vector"
                )
                .where(self.query_filter, prefilter=True)
                .limit(k)
                .to_list()
            )
        else:
            docs = (
                self.document_collection.search(
                    query=query_embedding, vector_column_name="vector"
                )
                .limit(k)
                .to_list()
            )
        return [
            VectorStoreSearchResult(
                document=VectorStoreDocument(
                    id=doc["id"],
                    text=doc["text"],
                    vector=doc["vector"],
                    attributes=json.loads(doc["attributes"]),
                ),
                score=1 - abs(float(doc["_distance"])),
            )
            for doc in docs
        ]

    fn similarity_search_by_text(
        self, text: str, text_embedder: TextEmbedder, k: int = 10, **kwargs: Any
    ) -> Vec<VectorStoreSearchResult>:
        /// Perform a similarity search using a given input text.
        query_embedding = text_embedder(text)
        if query_embedding:
            return self.similarity_search_by_vector(query_embedding, k)
        return []

    fn search_by_id(self, id: str) -> VectorStoreDocument:
        /// Search for a document by id.
        doc = (
            self.document_collection.search()
            .where(format!("id == '{id}'"), prefilter=True)
            .to_list()
        )
        if doc:
            return VectorStoreDocument(
                id=doc[0]["id"],
                text=doc[0]["text"],
                vector=doc[0]["vector"],
                attributes=json.loads(doc[0]["attributes"]),
            )
        return VectorStoreDocument(id=id, text=None, vector=None)
}
