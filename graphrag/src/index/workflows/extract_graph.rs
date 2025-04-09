//! A module containing run_workflow method definition.

use std::collections::HashMap;

use polars::prelude::LazyFrame;

use crate::cache::pipeline_cache::PipelineCache;
use crate::callbacks::workflow_callbacks::WorkflowCallbacks;
use crate::config::enums::AsyncType;
use crate::config::models::graph_rag_config::GraphRagConfig;
use crate::index::operations::extract_graph::extract_graph::extract_graph as extractor;
use crate::index::operations::summarize_descriptions::summarize_descriptions;
use crate::index::typing::context::PipelineRunContext;
use crate::index::typing::workflow::WorkflowFunctionOutput;
use crate::utils::storage::{load_table_from_storage, write_table_to_storage};


/// All the steps to create the base entity graph.
pub async fn run_workflow(
    config: GraphRagConfig,
    context: PipelineRunContext,
) -> WorkflowFunctionOutput {
    let text_units = load_table_from_storage("text_units", context.storage).await;

    let extract_graph_llm_settings = config.get_language_model_config(
        &config.extract_graph.model_id
    );
    let extraction_strategy = config.extract_graph.resolved_strategy(
        config.root_dir, extract_graph_llm_settings
    );

    let summarization_llm_settings = config.get_language_model_config(
        config.summarize_descriptions.model_id
    )
    let summarization_strategy = config.summarize_descriptions.resolved_strategy(
        config.root_dir, summarization_llm_settings
    );

    let (entities, relationships) = extract_graph(
        text_units=text_units,
        callbacks=context.callbacks,
        cache=context.cache,
        extraction_strategy=extraction_strategy,
        extraction_num_threads=extract_graph_llm_settings.concurrent_requests,
        extraction_async_mode=extract_graph_llm_settings.async_mode,
        entity_types=config.extract_graph.entity_types,
        summarization_strategy=summarization_strategy,
        summarization_num_threads=summarization_llm_settings.concurrent_requests,
    ).await;

    write_table_to_storage(entities, "entities", context.storage).await;
    write_table_to_storage(relationships, "relationships", context.storage).await;

    WorkflowFunctionOutput {
        result: {
            "entities": entities,
            "relationships": relationships,
        }
    }
}

/// All the steps to create the base entity graph.
pub async fn extract_graph(
    text_units: LazyFrame,
    callbacks: WorkflowCallbacks,
    cache: PipelineCache,
    extraction_strategy: Option<HashMap<String, Box<dyn Any>>>, // = None,
    extraction_num_threads: usize, // = 4,
    extraction_async_mode: AsyncType, // = AsyncType::AsyncIO,
    entity_types: Option<Vec<String>>, // = None,
    summarization_strategy: Option<HashMap<String, Box<dyn Any>>>, // = None,
    summarization_num_threads: usize, // = 4,
) -> (LazyFrame, LazyFrame) {
    // this returns a graph for each text unit, to be merged later
    let (extracted_entities, extracted_relationships) = extractor(
        text_units=text_units,
        callbacks=callbacks,
        cache=cache,
        text_column="text",
        id_column="id",
        strategy=extraction_strategy,
        async_mode=extraction_async_mode,
        entity_types=entity_types,
        num_threads=extraction_num_threads,
    ).await;

    if !_validate_data(extracted_entities) {
        error_msg = "Entity Extraction failed. No entities detected during extraction."
        callbacks.error(error_msg);
        raise ValueError(error_msg)
    }

    if !_validate_data(extracted_relationships) {
        error_msg = (
            "Entity Extraction failed. No relationships detected during extraction."
        );
        callbacks.error(error_msg);
        raise ValueError(error_msg)
    }

    let (entities, relationships) = get_summarized_entities_relationships(
        extracted_entities=extracted_entities,
        extracted_relationships=extracted_relationships,
        callbacks=callbacks,
        cache=cache,
        summarization_strategy=summarization_strategy,
        summarization_num_threads=summarization_num_threads,
    ).await;

    (entities, relationships)
}

/// Summarize the entities and relationships.
pub async fn get_summarized_entities_relationships(
    extracted_entities: LazyFrame,
    extracted_relationships: LazyFrame,
    callbacks: WorkflowCallbacks,
    cache: PipelineCache,
    summarization_strategy: Option<HashMap<String, Box<dyn Any>>>, // = None,
    summarization_num_threads: usize, // = 4,
) -> (LazyFrame, LazyFrame) {
    let (entity_summaries, relationship_summaries) = summarize_descriptions(
        extracted_entities,
        extracted_relationships,
        callbacks=callbacks,
        cache=cache,
        summarization_strategy,
        summarization_num_threads,
    ).await;

    let relationships = extracted_relationships.drop(columns=["description"]).join(
        relationship_summaries, on=["source", "target"], how="left"
    );

    let extracted_entities.drop(columns=["description"], inplace=true);
    let entities = extracted_entities.join(entity_summaries, on="title", how="left");
    (entities, relationships)
}

/// Validate that the LazyFrame has data.
fn _validate_data(df: LazyFrame) -> bool {
    !df.is_empty()
}
