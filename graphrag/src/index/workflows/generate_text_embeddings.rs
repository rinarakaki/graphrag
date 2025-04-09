//! A module containing run_workflow method definition.

use std::collections::{HashMap, HashSet};

use log::info;

use polars::prelude::LazyFrame;

use crate::cache::pipeline_cache::PipelineCache;
use crate::callbacks::workflow_callbacks::WorkflowCallbacks;
use crate::config::embeddings::{
    COMMUNITY_FULL_CONTENT_EMBEDDING,
    COMMUNITY_SUMMARY_EMBEDDING,
    COMMUNITY_TITLE_EMBEDDING,
    DOCUMENT_TEXT_EMBEDDING,
    ENTITY_DESCRIPTION_EMBEDDING,
    ENTITY_TITLE_EMBEDDING,
    get_embedded_fields,
    get_embedding_settings,
    RELATIONSHIP_DESCRIPTION_EMBEDDING,
    TEXT_UNIT_TEXT_EMBEDDING,
};
use crate::config::models::graph_rag_config::GraphRagConfig;
use crate::index::operations::embed_text::embed_text;
use crate::index::typing::context::PipelineRunContext;
use crate::index::typing::workflow::WorkflowFunctionOutput;
use crate::utils::storage::{load_table_from_storage, write_table_to_storage};

/// All the steps to transform community reports.
pub async fn run_workflow(
    config: GraphRagConfig,
    context: PipelineRunContext,
) -> WorkflowFunctionOutput {
    let documents = load_table_from_storage("documents", context.storage).await;
    let relationships = load_table_from_storage("relationships", context.storage).await;
    let text_units = load_table_from_storage("text_units", context.storage).await;
    let entities = load_table_from_storage("entities", context.storage).await;
    let community_reports = load_table_from_storage(
        "community_reports", context.storage
    ).await;

    let embedded_fields = get_embedded_fields(config);
    let text_embed = get_embedding_settings(config);

    let output = generate_text_embeddings(
        documents,
        relationships,
        text_units,
        entities,
        community_reports,
        context.callbacks,
        context.cache,
        text_embed,
        embedded_fields,
    ).await;

    if config.snapshots.embeddings {
        for (name, table) in output.items() {
            write_table_to_storage(
                table,
                format!("embeddings.{name}"),
                context.storage,
            ).await;
        }
    }

    WorkflowFunctionOutput {
        result: output
    }
}

/// All the steps to generate all embeddings.
pub async fn generate_text_embeddings(
    documents: Option<LazyFrame>,
    relationships: Option<LazyFrame>,
    text_units: Option<LazyFrame>,
    entities: Option<LazyFrame>,
    community_reports: Option<LazyFrame>,
    callbacks: WorkflowCallbacks,
    cache: PipelineCache,
    text_embed_config: dict,
    embedded_fields: HashSet<String>,
) -> HashMap<&str, LazyFrame> {
    let embedding_param_map = {
        DOCUMENT_TEXT_EMBEDDING: {
            "data": documents.loc[:, ["id", "text"]] if documents is not None else None,
            "embed_column": "text",
        },
        RELATIONSHIP_DESCRIPTION_EMBEDDING: {
            "data": relationships.loc[:, ["id", "description"]]
            if relationships is not None
            else None,
            "embed_column": "description",
        },
        TEXT_UNIT_TEXT_EMBEDDING: {
            "data": text_units.loc[:, ["id", "text"]]
            if text_units is not None
            else None,
            "embed_column": "text",
        },
        ENTITY_TITLE_EMBEDDING: {
            "data": entities.loc[:, ["id", "title"]] if entities is not None else None,
            "embed_column": "title",
        },
        ENTITY_DESCRIPTION_EMBEDDING: {
            "data": entities.loc[:, ["id", "title", "description"]].assign(
                title_description=lambda df: df["title"] + ":" + df["description"]
            )
            if entities is not None
            else None,
            "embed_column": "title_description",
        },
        COMMUNITY_TITLE_EMBEDDING: {
            "data": community_reports.loc[:, ["id", "title"]]
            if community_reports is not None
            else None,
            "embed_column": "title",
        },
        COMMUNITY_SUMMARY_EMBEDDING: {
            "data": community_reports.loc[:, ["id", "summary"]]
            if community_reports is not None
            else None,
            "embed_column": "summary",
        },
        COMMUNITY_FULL_CONTENT_EMBEDDING: {
            "data": community_reports.loc[:, ["id", "full_content"]]
            if community_reports is not None
            else None,
            "embed_column": "full_content",
        },
    };

    info!("Creating embeddings");
    let mut outputs = HashMap::new();
    for field in embedded_fields.iter() {
        outputs[field] = _run_and_snapshot_embeddings(
            field,
            callbacks,
            cache,
            text_embed_config,
            **embedding_param_map[field],
        ).await;
    }
    outputs
}

/// All the steps to generate single embedding.
async fn _run_and_snapshot_embeddings(
    name: &str,
    data: LazyFrame,
    embed_column: &str,
    callbacks: WorkflowCallbacks,
    cache: PipelineCache,
    text_embed_config: dict,
) -> LazyFrame {
    data["embedding"] = embed_text(
        data,
        callbacks,
        cache,
        embed_column,
        text_embed_config["strategy"],
        name,
    ).await;

    data[..][["id", "embedding"]]
}
