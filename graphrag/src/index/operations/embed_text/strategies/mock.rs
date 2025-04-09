//! A module containing run and _embed_text methods definitions.

use std::collections::HashMap;

use crate::cache::pipeline_cache::PipelineCache;
use crate::callbacks::workflow_callbacks::WorkflowCallbacks;
use crate::index::operations::embed_text::strategies::typing::TextEmbeddingResult;
use crate::logger::progress::{ProgressTicker, progress_ticker};

/// Run the Claim extraction chain.
pub async fn run<T>(
    input: Vec<String>,
    callbacks: impl WorkflowCallbacks,
    cache: impl PipelineCache<T>,
    _args: HashMap<String, String>,
) -> TextEmbeddingResult {
    let ticker = progress_ticker(Some(callbacks.progress), input.len());
    TextEmbeddingResult {
        embeddings: Some(input.iter().map(|text| {
            Some(_embed_text(cache, text, ticker))
        }).collect()),
    }
}

/// Embed a single piece of text.
fn _embed_text<T>(_cache: impl PipelineCache<T>, _text: &str, tick: ProgressTicker) -> Vec<f64> {
    tick(1);
    vec![random.random(), random.random(), random.random()]
}
