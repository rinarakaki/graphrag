//! A module containing 'TextEmbeddingResult' model.

use std::collections::HashMap;

use crate::cache::pipeline_cache::PipelineCache;
use crate::callbacks::workflow_callbacks::WorkflowCallbacks;

/// Text embedding result class definition.
pub struct TextEmbeddingResult {
    embeddings: Option<Vec<Option<Vec<f64>>>>,
}

pub type TextEmbeddingStrategy = fn(
    Vec<String>,
    impl WorkflowCallbacks,
    impl PipelineCache,
    HashMap<String, String>,
) -> impl Future<Output = TextEmbeddingResult>;
