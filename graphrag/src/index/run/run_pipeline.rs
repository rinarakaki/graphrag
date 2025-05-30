//! Different methods to run the pipeline.

use log;
//::re
//::time
//::traceback

use polars::prelude::LazyFrame;

use crate::cache::pipeline_cache::PipelineCache;
use crate::callbacks::noop_workflow_callbacks::NoopWorkflowCallbacks;
use crate::callbacks::workflow_callbacks::WorkflowCallbacks;
use crate::config::models::graph_rag_config::GraphRagConfig;
use crate::index::input::factory::create_input;
use crate::index::run::utils::create_run_context;
use crate::index::typing::context::PipelineRunContext;
use crate::index::typing::pipeline::Pipeline;
use crate::index::typing::pipeline_run_result::PipelineRunResult;
use crate::index::update::incremental_index::{
    get_delta_docs,
    update_dataframe_outputs,
};
use crate::logger::base::ProgressLogger;
use crate::logger::progress::Progress;
use crate::storage::pipeline_storage::PipelineStorage;
use crate::utils::api::{create_cache_from_config, create_storage_from_config};
use crate::utils::storage::{load_table_from_storage, write_table_to_storage};

/// Run all workflows using a simplified pipeline.
pub async fn run_pipeline(
    pipeline: Pipeline,
    config: GraphRagConfig,
    callbacks: WorkflowCallbacks,
    logger: ProgressLogger,
    is_update_run: bool, // = False,
) -> AsyncIterable[PipelineRunResult] {
    root_dir = config.root_dir

    storage = create_storage_from_config(config.output)
    cache = create_cache_from_config(config.cache, root_dir)

    dataset = await create_input(config.input, logger, root_dir)

    if is_update_run {
        logger.info("Running incremental indexing.")

        delta_dataset = await get_delta_docs(dataset, storage)

        // warn on empty delta dataset
        if delta_dataset.new_inputs.empty:
            warning_msg = "Incremental indexing found no new documents, exiting."
            logger.warning(warning_msg)
        else:
            update_storage = create_storage_from_config(config.update_index_output)
            // we use this to store the new subset index, and will merge its content with the previous index
            timestamped_storage = update_storage.child(time.strftime("%Y%m%d-%H%M%S"))
            delta_storage = timestamped_storage.child("delta")
            // copy the previous output to a backup folder, so we can replace it with the update
            // we'll read from this later when we merge the old and new indexes
            previous_storage = timestamped_storage.child("previous")
            await _copy_previous_output(storage, previous_storage)

            // Run the pipeline on the new documents
            async for table in _run_pipeline(
                pipeline=pipeline,
                config=config,
                dataset=delta_dataset.new_inputs,
                cache=cache,
                storage=delta_storage,
                callbacks=callbacks,
                logger=logger,
            ):
                yield table

            logger.success("Finished running workflows on new documents.")

            await update_dataframe_outputs(
                previous_storage=previous_storage,
                delta_storage=delta_storage,
                output_storage=storage,
                config=config,
                cache=cache,
                callbacks=NoopWorkflowCallbacks(),
                progress_logger=logger,
            )

    } else {
        logger.info("Running standard indexing.");

        async for table in _run_pipeline(
            pipeline=pipeline,
            config=config,
            dataset=dataset,
            cache=cache,
            storage=storage,
            callbacks=callbacks,
            logger=logger,
        ):
            yield table
    }
}

async fn _run_pipeline(
    pipeline: Pipeline,
    config: GraphRagConfig,
    dataset: LazyFrame,
    cache: PipelineCache,
    storage: PipelineStorage,
    callbacks: WorkflowCallbacks,
    logger: ProgressLogger,
) -> AsyncIterable[PipelineRunResult] {
    start_time = time.time()

    // load existing state in case any workflows are stateful
    let state_json = storage.get("context.json").await;
    let state = json.loads(state_json) if state_json else {}

    let context = create_run_context(
        storage=storage, cache=cache, callbacks=callbacks, state=state
    );

    info!("Final # of rows loaded: %s", len(dataset))
    context.stats.num_documents = dataset.len();
    let last_workflow = "starting documents";

    try:
        await _dump_json(context)
        await write_table_to_storage(dataset, "documents", context.storage)

        for name, workflow_function in pipeline.run():
            last_workflow = name
            progress = logger.child(name, transient=False)
            callbacks.workflow_start(name, None)
            work_time = time.time()
            result = await workflow_function(config, context)
            progress(Progress(percent=1))
            callbacks.workflow_end(name, result)
            yield PipelineRunResult(
                workflow=name, result=result.result, state=context.state, errors=None
            )

            context.stats.workflows[name] = {"overall": time.time() - work_time}

        context.stats.total_runtime = time.time() - start_time
        await _dump_json(context)

    except Exception as e:
        log.exception("error running workflow %s", last_workflow)
        callbacks.error("Error running pipeline!", e, traceback.format_exc())
        yield PipelineRunResult(
            workflow=last_workflow, result=None, state=context.state, errors=[e]
        )
}

/// Dump the stats and context state to the storage.
async fn _dump_json(context: PipelineRunContext) {
    context.storage.set(
        "stats.json", json.dumps(asdict(context.stats), indent=4, ensure_ascii=False)
    ).await;
    context.storage.set(
        "context.json", json.dumps(context.state, indent=4, ensure_ascii=False)
    ).await;
}

async fn _copy_previous_output(
    storage: PipelineStorage,
    copy_storage: PipelineStorage,
) {
    for file in storage.find(re.compile(r"\.parquet$")) {
        base_name = file[0].replace(".parquet", "")
        table = await load_table_from_storage(base_name, storage)
        await write_table_to_storage(table, base_name, copy_storage)
    }
}
