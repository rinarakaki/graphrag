//! Indexing API for GraphRAG.
//!
//! WARNING: This API is under development and may undergo changes in future releases.
//! Backwards compatibility is not guaranteed at this time.

use log::warn;

use crate::callbacks::reporting::create_pipeline_reporter;
use crate::callbacks::workflow_callbacks::WorkflowCallbacks;
use crate::config::enums::IndexingMethod;
use crate::config::models::graph_rag_config::GraphRagConfig;
use crate::index::run::run_pipeline::run_pipeline;
use crate::index::run::utils::create_callback_chain;
use crate::index::typing::pipeline_run_result::PipelineRunResult;
use crate::index::typing::workflow::WorkflowFunction;
use crate::index::workflows::factory::PipelineFactory;
use crate::logger::base::ProgressLogger;
use crate::logger::null_progress::NullProgressLogger;

/**
Run the pipeline with the given configuration.

Parameters
----------
config : GraphRagConfig
    The configuration.
method : IndexingMethod default=IndexingMethod.Standard
    Styling of indexing to perform (full LLM, NLP + LLM, etc.).
memory_profile : bool
    Whether to enable memory profiling.
callbacks : Vec<WorkflowCallbacks> | None default=None
    A list of callbacks to register.
progress_logger : ProgressLogger | None default=None
    The progress logger.

Returns
-------
Vec<PipelineRunResult>
    The list of pipeline run results
 */
pub async fn build_index(
    config: GraphRagConfig,
    method: IndexingMethod, // = IndexingMethod.Standard,
    is_update_run: bool, // = False,
    memory_profile: bool, // = False,
    callbacks: Option<Vec<impl WorkflowCallbacks>>, // = None,
    progress_logger: Option<impl ProgressLogger>, // = None,
) -> Vec<PipelineRunResult> {
    let logger = progress_logger.unwrap_or(NullProgressLogger);
    // create a pipeline reporter and add to any additional callbacks
    let callbacks = callbacks.unwrap_or_default();
    callbacks.push(create_pipeline_reporter(config.reporting, None));

    let workflow_callbacks = create_callback_chain(callbacks, logger);

    let outputs = Vec::<PipelineRunResult>::new();

    if memory_profile {
        warn!("New pipeline does not yet support memory profiling.");
    }

    let pipeline = PipelineFactory.create_pipeline(config, method);

    workflow_callbacks.pipeline_start(pipeline.names());

    async for output in run_pipeline(
        pipeline,
        config,
        callbacks=workflow_callbacks,
        logger=logger,
        is_update_run=is_update_run,
    ) {
        outputs.push(output);
        if output.errors and len(output.errors) > 0{
            logger.error(output.workflow)
        }else{
            logger.success(output.workflow)}
        logger.info(str(output.result))
    }

    workflow_callbacks.pipeline_end(outputs);
    outputs
}

/// Register a custom workflow function. You can then include the name in the settings.yaml workflows list.
pub fn register_workflow_function(name: str, workflow: impl WorkflowFunction) {
    PipelineFactory.register(name, workflow)
}
