//! A module containing embeddings values.

use std::collections::{HashMap, HashSet};

use crate::config::enums::TextEmbeddingTarget;
use crate::config::models::graph_rag_config::GraphRagConfig;

pub const ENTITY_TITLE_EMBEDDING: &str = "entity.title";
pub const ENTITY_DESCRIPTION_EMBEDDING: &str = "entity.description";
pub const RELATIONSHIP_DESCRIPTION_EMBEDDING: &str = "relationship.description";
pub const DOCUMENT_TEXT_EMBEDDING: &str = "document.text";
pub const COMMUNITY_TITLE_EMBEDDING: &str = "community.title";
pub const COMMUNITY_SUMMARY_EMBEDDING: &str = "community.summary";
pub const COMMUNITY_FULL_CONTENT_EMBEDDING: &str = "community.full_content";
pub const TEXT_UNIT_TEXT_EMBEDDING: &str = "text_unit.text";

const ALL_EMBEDDINGS: HashSet<&str> = HashSet::from([
    ENTITY_TITLE_EMBEDDING,
    ENTITY_DESCRIPTION_EMBEDDING,
    RELATIONSHIP_DESCRIPTION_EMBEDDING,
    DOCUMENT_TEXT_EMBEDDING,
    COMMUNITY_TITLE_EMBEDDING,
    COMMUNITY_SUMMARY_EMBEDDING,
    COMMUNITY_FULL_CONTENT_EMBEDDING,
    TEXT_UNIT_TEXT_EMBEDDING,
]);
const REQUIRED_EMBEDDINGS: HashSet<&str> = HashSet::from([
    ENTITY_DESCRIPTION_EMBEDDING,
    COMMUNITY_FULL_CONTENT_EMBEDDING,
    TEXT_UNIT_TEXT_EMBEDDING,
]);

/// Get the fields to embed based on the enum or specifically selected embeddings.
pub fn get_embedded_fields(settings: GraphRagConfig) -> HashSet<&str> {
    match settings.embed_text.target {
        TextEmbeddingTarget::All => ALL_EMBEDDINGS,
        TextEmbeddingTarget::Required => REQUIRED_EMBEDDINGS,
        TextEmbeddingTarget::Selected => HashSet::from(settings.embed_text.names),
        TextEmbeddingTarget::None => HashSet::new(),
    }
}

/// Transform GraphRAG config into settings for workflows.
pub fn get_embedding_settings(
    settings: GraphRagConfig,
    vector_store_params: Option<HashMap<String, String>>,
) -> Option<HashMap<String, HashMap<String, String>>> {
    // TEMP
    let embeddings_llm_settings = settings.get_language_model_config(
        settings.embed_text.model_id.as_str(),
    );
    let vector_store_settings = settings.get_vector_store_config(
        settings.embed_text.vector_store_id.as_str(),
    ).model_dump();

    //
    // If we get to this point, settings.vector_store is defined, and there's a specific setting for this embedding.
    // settings.vector_store.base contains connection information, or may be undefined
    // settings.vector_store.<vector_name> contains the specific settings for this embedding
    //
    let strategy = settings.embed_text.resolved_strategy(
        embeddings_llm_settings
    );  // get the default strategy
    strategy.update({
        "vector_store": {
            **(vector_store_params or {}),
            **(vector_store_settings),
        }
    });  // update the default strategy with the vector store settings
    // This ensures the vector store config is part of the strategy and not the global config
    Some(HashMap::from[("strategy", strategy)])
}

/**
Create a collection name for the embedding store.

Within any given vector store, we can have multiple sets of embeddings organized into projects.
The `container` param is used for this partitioning, and is added as a prefix to the collection name for differentiation.

The embedding name is fixed, with the available list defined in graphrag.index.config.embeddings

Note that we use dot notation in our names, but many vector stores do not support this - so we convert to dashes.
 */
pub fn create_collection_name(
    container_name: &str, embedding_name: &str, validate: bool,  // TODO(rinarakaki) = true
) -> String {
    assert!(
        !validate || ALL_EMBEDDINGS.contains(&embedding_name),
    "Invalid embedding name: {embedding_name}",
    );
    format!("{container_name}-{embedding_name}").replace(".", "-")
}
