//! A module containing the WorkflowCallbacks registry.

use std::collections::HashMap;

use crate::callbacks::workflow_callbacks::WorkflowCallbacks;
use crate::index::typing::pipeline_run_result::PipelineRunResult;
use crate::logger::progress::Progress;

/// A registry of WorkflowCallbacks.
pub struct WorkflowCallbacksManager {
    callbacks: Vec<Box<dyn WorkflowCallbacks>>,
}

impl WorkflowCallbacksManager {
    /// Create a new instance of WorkflowCallbacksRegistry.
    pub fn new() -> Self {
        WorkflowCallbacksManager {
            callbacks: Vec::new(),
        }
    }

    /// Register a new WorkflowCallbacks type.
    pub fn register(&mut self, callbacks: Box<dyn WorkflowCallbacks>) {
        self.callbacks.push(callbacks);
    }
}

impl WorkflowCallbacks for WorkflowCallbacksManager {
    /// Execute this callback when a the entire pipeline starts.
    fn pipeline_start(&self, names: Vec<String>) {
        for callback in &self.callbacks {
            callback.pipeline_start(names.clone());
        }
    }

    /// Execute this callback when the entire pipeline ends.
    fn pipeline_end(&self, results: Vec<PipelineRunResult>) {
        for callback in &self.callbacks {
            callback.pipeline_end(results.clone());
        }
    }

    /// Execute this callback when a workflow starts.
    fn workflow_start(&mut self, name: &str, instance: Option<()>) {
        for callback in &mut self.callbacks {
            callback.workflow_start(name.clone(), instance);
        }
    }

    /// Execute this callback when a workflow ends.
    fn workflow_end(&mut self, name: &str, instance: Option<()>) {
        for callback in &mut self.callbacks {
            callback.workflow_end(name.clone(), instance);
        }
    }

    /// Handle when progress occurs.
    fn progress(&self, progress: Progress) {
        for callback in &self.callbacks {
            callback.progress(progress.clone());
        }
    }

    /// Handle when an error occurs.
    fn error(
        &self,
        message: String,
        cause: Option<Box<dyn std::error::Error>>,
        stack: Option<String>,
        details: Option<HashMap<String, String>>,
    ) {
        for callback in &self.callbacks {
            callback.error(
                message.clone(),
                cause.clone(),
                stack.clone(),
                details.clone(),
            );
        }
    }

    /// Handle when a warning occurs.
    fn warning(&self, message: String, details: Option<HashMap<String, String>>) {
        for callback in &self.callbacks {
            callback.warning(message.clone(), details.clone());
        }
    }

    /// Handle when a log message occurs.
    fn log(&self, message: String, details: Option<HashMap<String, String>>) {
        for callback in &self.callbacks {
            callback.log(message.clone(), details.clone());
        }
    }
}
