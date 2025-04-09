//! Collection of callbacks that can be used to monitor the workflow execution.

use std::collections::HashMap;

use crate::index::typing::pipeline_run_result::PipelineRunResult;
use crate::logger::progress::Progress;

/// A collection of callbacks that can be used to monitor the workflow execution.
///
/// This base class is a "noop" implementation so that clients may implement just the callbacks they need.
#[allow(unused_variables)]
pub trait WorkflowCallbacks {
    /// Execute this callback to signal when the entire pipeline starts.
    fn pipeline_start(&self, names: Vec<String>) {}

    /// Execute this callback to signal when the entire pipeline ends.
    fn pipeline_end(&mut self, results: Vec<PipelineRunResult>) {}

    /// Execute this callback when a workflow starts.
    fn workflow_start(&mut self, name: &str, instance: Option<()>) {}

    /// Execute this callback when a workflow ends.
    fn workflow_end(&mut self, name: &str, instance: Option<()>) {}

    /// Handle when progress occurs.
    fn progress(&self, progress: Progress) {}

    /// Handle when an error occurs.
    fn error(
        &self,
        message: String,
        cause: Option<Box<dyn std::error::Error>>,
        stack: Option<String>,
        details: Option<HashMap<String, String>>,
    ) {
    }

    /// Handle when a warning occurs.
    fn warning(&self, message: String, details: Option<HashMap<String, String>>) {}

    /// Handle when a log message occurs.
    fn log(&self, message: String, details: Option<HashMap<String, String>>) {}
}
