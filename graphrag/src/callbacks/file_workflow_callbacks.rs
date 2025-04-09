//! A logger that emits updates from the indexing engine to a local file.

use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::path::Path;

use log::{info, warn};

use crate::callbacks::workflow_callbacks::WorkflowCallbacks;

/// A logger that writes to a local file.
pub struct FileWorkflowCallbacks {
    _out_stream: BufWriter<std::fs::File>,
}
impl FileWorkflowCallbacks {
    /// Create a new file-based workflow logger.
    pub fn new(directory: &str) -> Self {
        std::fs::create_dir_all(directory).unwrap();
        let path = Path::new(directory).join("logs.json");
        let file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .unwrap();
        FileWorkflowCallbacks {
            _out_stream: BufWriter::new(file),
        }
    }
}

impl WorkflowCallbacks for FileWorkflowCallbacks {
    /// Handle when an error occurs.
    fn error(
        &self,
        message: String,
        cause: Option<Box<dyn std::error::Error>>,
        stack: Option<String>,
        details: Option<HashMap<String, String>>,
    ) {
        self._out_stream.write(
            serde_json::to_string(HashMap::from([
                ("type", "error"),
                ("data", &message),
                ("stack", stack),
                ("source", cause.map(|e| e.to_string())),
                ("details", details),
            ]))
            .unwrap()
                + "\n",
        );
        info!("{message} details={details}");
    }

    /// Handle when a warning occurs.
    fn warning(&self, message: String, details: Option<HashMap<String, String>>) {
        self._out_stream.write(
            serde_json::to_string(HashMap::from([
                ("type", "warning"),
                ("data", &message),
                ("details", details),
            ]))
            .unwrap()
                + "\n",
        );
        _print_warning(message);
    }

    /// Handle when a log message is produced.
    fn log(&self, message: String, details: Option<HashMap<String, String>>) {
        self._out_stream.write(
            serde_json::to_string(&HashMap::from([
                ("type", "log"),
                ("data", &message),
                ("details", details),
            ]))
            .unwrap()
                + "\n",
        );

        info!("{message} details={details:?}");
    }
}

fn _print_warning(skk: String) {
    warn!("{}", skk);
}
