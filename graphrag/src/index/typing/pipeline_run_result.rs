//! A module containing the PipelineRunResult class.

use std::any::Any;

use crate::index::typing::state::PipelineState;

/// Pipeline run result class definition.
pub struct PipelineRunResult {
    /// The name of the workflow that was executed.
    workflow: String,
    /// The result of the workflow function. This can be anything - we use it only for logging downstream, and expect each workflow function to write official outputs to the provided storage.
    result: Option<Any>,
    /// Ongoing pipeline context state object.
    state: PipelineState,
    errors: Option<Vec<dyn std::error::Error>>,
}
