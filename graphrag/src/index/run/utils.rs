//! Utility functions for the GraphRAG run module.

use crate::cache::memory_pipeline_cache::InMemoryCache;
use crate::cache::pipeline_cache::PipelineCache;
use crate::callbacks::noop_workflow_callbacks::NoopWorkflowCallbacks;
use crate::callbacks::progress_workflow_callbacks::ProgressWorkflowCallbacks;
use crate::callbacks::workflow_callbacks::WorkflowCallbacks;
use crate::callbacks::workflow_callbacks_manager::WorkflowCallbacksManager;
use crate::index::typing::context::PipelineRunContext;
use crate::index::typing::state::PipelineState;
use crate::index::typing::stats::PipelineRunStats;
use crate::logger::base::ProgressLogger;
use crate::storage::memory_pipeline_storage::MemoryPipelineStorage;
use crate::storage::pipeline_storage::PipelineStorage;

/// Create the run context for the pipeline.
pub fn create_run_context<T>(
    storage: Option<impl PipelineStorage<T>>, // = None
    cache: Option<impl PipelineCache<T>>, // = None
    callbacks: Option<impl WorkflowCallbacks>, // = None
    stats: Option<PipelineRunStats>, // = None
    state: Option<PipelineState>, // = None
) -> PipelineRunContext {
    PipelineRunContext {
        stats: stats.unwrap_or_else(|| PipelineRunStats::new()),
        cache: cache.unwrap_or_else(|| InMemoryCache::new()),
        storage: storage.unwrap_or_else(|| MemoryPipelineStorage::new()),
        callbacks: callbacks.unwrap_or_else(|| NoopWorkflowCallbacks::new()),
        state: state.unwrap_or_default(),
    }
}

/// Create a callback manager that encompasses multiple callbacks.
pub fn create_callback_chain(
    callbacks: Option<Vec<impl WorkflowCallbacks>>,
    progress: Option<impl ProgressLogger>,
) -> impl WorkflowCallbacks {
    let mut manager = WorkflowCallbacksManager::new();
    if let Some(callbacks_vec) = callbacks {
        for callback in callbacks_vec {
            manager.register(callback);
        }
    }
    if let Some(progress) = progress {
        manager.register(ProgressWorkflowCallbacks::new(progress));
    }
    manager
}
