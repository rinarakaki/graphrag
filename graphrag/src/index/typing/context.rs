//! A module containing the 'PipelineRunContext' models.

use crate::cache::pipeline_cache::PipelineCache;
use crate::callbacks::workflow_callbacks::WorkflowCallbacks;
use crate::index::typing::state::PipelineState;
use crate::index::typing::stats::PipelineRunStats;
use crate::storage::pipeline_storage::PipelineStorage;

/// Provides the context for the current pipeline run.
pub struct PipelineRunContext<
    T,
    Storage: PipelineStorage<T>,
    Cache: PipelineCache<T>,
    Callbacks: WorkflowCallbacks,
> {
    pub stats: PipelineRunStats,
    /// Long-term storage for pipeline verbs to use. Items written here will be written to the storage provider.
    pub storage: Storage,
    /// Cache instance for reading previous LLM responses.
    pub cache: Cache,
    /// Callbacks to be called during the pipeline run.
    pub callbacks: Callbacks,
    /// Arbitrary property bag for runtime state, persistent pre-computes, or experimental features.
    pub state: PipelineState,
}
