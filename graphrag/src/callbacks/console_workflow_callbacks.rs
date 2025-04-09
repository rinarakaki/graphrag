//! A logger that emits updates from the indexing engine to the console.

use std::collections::HashMap;

use crate::callbacks::workflow_callbacks::WorkflowCallbacks;

/// A logger that writes to a console.
pub struct ConsoleWorkflowCallbacks;

#[allow(unused_variables)]
impl WorkflowCallbacks for ConsoleWorkflowCallbacks {
    /// Handle when an error occurs.
    fn error(
        &self,
        message: String,
        cause: Option<Box<dyn std::error::Error>>,
        stack: Option<String>,
        details: Option<HashMap<String, String>>,
    ) {
        print!("{} {:?}, {:?}, {:?}", message, cause, stack, details);
    }

    /// Handle when a warning occurs.
    fn warning(&self, message: String, details: Option<HashMap<String, String>>) {
        _print_warning(message);
    }

    /// Handle when a log message is produced.
    fn log(&self, message: String, details: Option<HashMap<String, String>>) {
        print!("{}, {:?}", message, details);
    }
}

fn _print_warning(skk: String) {
    print!("\033[93m {}\033[00m", skk);
}
