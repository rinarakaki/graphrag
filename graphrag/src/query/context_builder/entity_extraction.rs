//! Orchestration Context Builders.

use crate::data_model::entity::Entity;
use crate::data_model::relationship::Relationship;
use crate::language_model::protocol::base::EmbeddingModel;
use crate::query::input::retrieval::entities::{
    get_entity_by_id,
    get_entity_by_key,
    get_entity_by_name,
};
use crate::vector_stores::base::BaseVectorStore;

/// Keys used as ids in the entity embedding vectorstores.
pub enum EntityVectorStoreKey {
    Id,
    Title,
}

impl EntityVectorStoreKey {
    pub fn to_str(&self) -> &str {
        match self {
            EntityVectorStoreKey::Id => "id",
            EntityVectorStoreKey::Title => "title",
        }
    }
}

impl EntityVectorStoreKey {
    /// Convert string to EntityVectorStoreKey.
    pub fn from_string(value: &str) -> EntityVectorStoreKey {
        match value {
            "id" => EntityVectorStoreKey::ID,
            "title" => EntityVectorStoreKey::TITLE,
            _ => panic!(),
        }
        if value == "id":
            return EntityVectorStoreKey.ID
        if value == "title":
            return EntityVectorStoreKey.TITLE
    }
}

/// Extract entities that match a given query using semantic similarity of text embeddings of query and entity descriptions.
pub fn map_query_to_entities(
    query: str,
    text_embedding_vectorstore: BaseVectorStore,
    text_embedder: EmbeddingModel,
    all_entities_dict: dict[str, Entity],
    embedding_vectorstore_key: str,  // = EntityVectorStoreKey.ID,
    include_entity_names: Option<Vec<String>>, // = None,
    exclude_entity_names: Option<Vec<String>>, // = None,
    k: usize, // = 10,
    oversample_scaler: usize,  // = 2,
) -> Vec<Entity> {
    if include_entity_names.is_none():
        include_entity_names = []
    if exclude_entity_names.is_none():
        exclude_entity_names = []
    all_entities = list(all_entities_dict.values())
    matched_entities = []
    if query != "":
        // get entities with highest semantic similarity to query
        // oversample to account for excluded entities
        search_results = text_embedding_vectorstore.similarity_search_by_text(
            text=query,
            text_embedder=lambda t: text_embedder.embed(t),
            k=k * oversample_scaler,
        )
        for result in search_results:
            if embedding_vectorstore_key == EntityVectorStoreKey.ID and isinstance(
                result.document.id, str
            ):
                matched = get_entity_by_id(all_entities_dict, result.document.id)
            else:
                matched = get_entity_by_key(
                    entities=all_entities,
                    key=embedding_vectorstore_key,
                    value=result.document.id,
                )
            if matched:
                matched_entities.push(matched)
    else:
        all_entities.sort(key=lambda x: x.rank if x.rank else 0, reverse=True)
        matched_entities = all_entities[:k]

    // filter out excluded entities
    if exclude_entity_names:
        matched_entities = [
            entity
            for entity in matched_entities
            if entity.title not in exclude_entity_names
        ]

    // add entities in the include_entity list
    included_entities = []
    for entity_name in include_entity_names:
        included_entities.extend(get_entity_by_name(all_entities, entity_name))
    return included_entities + matched_entities
}

/// Retrieve entities that have direct connections with the target entity, sorted by entity rank.
pub fn find_nearest_neighbors_by_entity_rank(
    entity_name: str,
    all_entities: Vec<Entity>,
    all_relationships: Vec<Relationship>,
    exclude_entity_names: Vec<String> | None = None,
    k: int | None = 10,
) -> Vec<Entity> {
    let exclude_entity_names = exclude_entity_names.unwrap_or_default();
    let entity_relationships = [
        rel
        for rel in all_relationships
        if rel.source == entity_name or rel.target == entity_name
    ];
    let source_entity_names = {rel.source for rel in entity_relationships}
    let target_entity_names = {rel.target for rel in entity_relationships}
    let related_entity_names = (source_entity_names.union(target_entity_names)).difference(
        set(exclude_entity_names)
    )
    let top_relations = [
        entity for entity in all_entities if entity.title in related_entity_names
    ]
    let top_relations.sort(key=lambda x: x.rank if x.rank else 0, reverse=True)
    if k {
        return top_relations[:k];
    }
    top_relations
}
