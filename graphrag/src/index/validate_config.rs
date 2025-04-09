//! A module containing validate_config_names definition.

use crate::callbacks::noop_workflow_callbacks::NoopWorkflowCallbacks;
use crate::config::defaults::language_model_defaults;
use crate::config::models::graph_rag_config::GraphRagConfig;
use crate::language_model::manager::ModelManager;
use crate::logger::print_progress::ProgressLogger;

/// Validate config file for LLM deployment name typos.
pub fn validate_config_names(logger: ProgressLogger, parameters: GraphRagConfig) {
    // Validate Chat LLM configs
    // TODO: Replace default_chat_model with a way to select the model
    let default_llm_settings = parameters.get_language_model_config("default_chat_model")
    // if max_retries is not set, set it to the default value
    if default_llm_settings.max_retries == -1 {
        default_llm_settings.max_retries = language_model_defaults.max_retries
    }
    let llm = ModelManager().register_chat(
        name="test-llm",
        model_type=default_llm_settings.type,
        config=default_llm_settings,
        callbacks=NoopWorkflowCallbacks(),
        cache=None,
    )

    try:
        asyncio.run(llm.achat("This is an LLM connectivity test. Say Hello World"))
        logger.success("LLM Config Params Validated")
    except Exception as e:
        logger.error(f"LLM configuration error detected. Exiting...\n{e}")
        sys.exit(1)

    // Validate Embeddings LLM configs
    embedding_llm_settings = parameters.get_language_model_config(
        parameters.embed_text.model_id
    )
    if embedding_llm_settings.max_retries == -1:
        embedding_llm_settings.max_retries = language_model_defaults.max_retries
    embed_llm = ModelManager().register_embedding(
        name="test-embed-llm",
        model_type=embedding_llm_settings.type,
        config=embedding_llm_settings,
        callbacks=NoopWorkflowCallbacks(),
        cache=None,
    )

    try:
        asyncio.run(embed_llm.aembed_batch(["This is an LLM Embedding Test String"]))
        logger.success("Embedding LLM Config Params Validated")
    except Exception as e:
        logger.error(f"Embedding LLM configuration error detected. Exiting...\n{e}")
        sys.exit(1)
}
