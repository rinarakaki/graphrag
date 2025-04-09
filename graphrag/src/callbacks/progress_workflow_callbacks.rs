//! A workflow callback manager that emits updates.

use crate::callbacks::workflow_callbacks::WorkflowCallbacks;
use crate::logger::base::ProgressLogger;
use crate::logger::progress::Progress;

/// A callbackmanager that delegates to a ProgressLogger.
pub struct ProgressWorkflowCallbacks<Logger: ProgressLogger> {
    _root_progress: Logger,
    _progress_stack: Vec<Logger>,
}

impl<Logger: ProgressLogger> ProgressWorkflowCallbacks<Logger> {
    /// Create a new ProgressWorkflowCallbacks.
    pub fn new(progress: Logger) -> Self {
        ProgressWorkflowCallbacks {
            _root_progress: progress,
            _progress_stack: vec![progress],
        }
    }

    fn _pop(&mut self) {
        self._progress_stack.pop();
    }

    fn _push(&mut self, name: &str) {
        self._progress_stack.push(self._latest().child(name, true));
    }

    // @property
    fn _latest(&self) -> &Logger {
        self._progress_stack.last().unwrap()
    }
}

#[allow(unused_variables)]
impl<Logger: ProgressLogger> WorkflowCallbacks for ProgressWorkflowCallbacks<Logger> {
    /// Execute this callback when a workflow starts.
    fn workflow_start(&mut self, name: &str, instance: Option<()>) {
        self._push(name);
    }

    /// Execute this callback when a workflow ends.
    fn workflow_end(&mut self, name: &str, instance: Option<()>) {
        self._pop();
    }

    /// Handle when progress occurs.
    fn progress(&self, progress: Progress) {
        self._latest()(progress);
    }
}
