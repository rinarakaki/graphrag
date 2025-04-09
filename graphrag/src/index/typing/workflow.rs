//! Pipeline workflow types.

use crate::config::models::graph_rag_config::GraphRagConfig;
use crate::index::typing::context::PipelineRunContext;

/// Data container for Workflow function results.
pub struct WorkflowFunctionOutput<T> {
    /// The result of the workflow function. This can be anything - we use it only for logging downstream, and expect each workflow function to write official outputs to the provided storage.
    result: Option<T>,
}

pub type WorkflowFunction<T> =
    fn(GraphRagConfig, PipelineRunContext) -> Awaitable<WorkflowFunctionOutput<T>>;
pub type Workflow<T> = (String, WorkflowFunction<T>);
