use graphrag::config::create_graphrag_config::create_graphrag_config;
use graphrag::index::workflows::create_base_text_units::run_workflow;
use graphrag::utils::storage::load_table_from_storage;

use super::util::{
    DEFAULT_MODEL_CONFIG,
    compare_outputs,
    create_test_context,
    load_test_table,
    update_document_metadata,
};

#[test]
async fn test_create_base_text_units() {
    let expected = load_test_table("text_units");

    let context = create_test_context().await;

    let config = create_graphrag_config({"models": DEFAULT_MODEL_CONFIG});

    await run_workflow(config, context);

    let actual = load_table_from_storage("text_units", context.storage).await;

    compare_outputs(actual, expected, columns=["text", "document_ids", "n_tokens"]);
}

#[test]
async fn test_create_base_text_units_metadata() {
    let expected = load_test_table("text_units_metadata");

    let context = create_test_context().await;

    let mut config = create_graphrag_config({"models": DEFAULT_MODEL_CONFIG});
    // test data was created with 4o, so we need to match the encoding for chunks to be identical
    config.chunks.encoding_model = "o200k_base";
    config.input.metadata = ["title"];
    config.chunks.prepend_metadata = True;

    await update_document_metadata(config.input.metadata, context);

    await run_workflow(config, context);

    let actual = load_table_from_storage("text_units", context.storage).await;
    compare_outputs(actual, expected);
}

#[test]
async fn test_create_base_text_units_metadata_included_in_chunk() {
    let expected = load_test_table("text_units_metadata_included_chunk");

    let context = create_test_context().await;

    let mut config = create_graphrag_config({"models": DEFAULT_MODEL_CONFIG});
    // test data was created with 4o, so we need to match the encoding for chunks to be identical
    config.chunks.encoding_model = "o200k_base";
    config.input.metadata = ["title"];
    config.chunks.prepend_metadata = True;
    config.chunks.chunk_size_includes_metadata = True;

    update_document_metadata(config.input.metadata, context).await;

    run_workflow(config, context).await;

    let actual = load_table_from_storage("text_units", context.storage).await;
    // only check the columns from the base workflow - our expected table is the final and will have more
    compare_outputs(actual, expected, columns=["text", "document_ids", "n_tokens"]);
}
