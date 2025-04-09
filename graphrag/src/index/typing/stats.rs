//! Pipeline stats types.

use std::collections::HashMap;

/// Pipeline running stats.
#[derive(Default)]
pub struct PipelineRunStats {
    /// Float representing the total runtime.
    total_runtime: f64,

    /// Number of documents.
    num_documents: usize,

    /// Float representing the input load time.
    input_load_time: f64,

    /// A dictionary of workflows.
    workflows: HashMap<String, HashMap<String, f64>>,
}
