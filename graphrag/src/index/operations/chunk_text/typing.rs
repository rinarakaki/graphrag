//! A module containing 'TextChunk' model.

use crate::config::models::chunking_config::ChunkingConfig;
use crate::logger::progress::ProgressTicker;

/// Text chunk class definition.
pub struct TextChunk {
    pub text_chunk: String,
    pub source_doc_indices: Vec<usize>,
    pub n_tokens: Option<usize>,
}

/// Input to a chunking strategy. Can be a string, a list of strings, or a list of tuples of (id, text).
pub enum ChunkInput {
    String(String),
    VecOfStrings(Vec<String>),
    VecOfTuples(Vec<(String, String)>),
}

pub type ChunkStrategy = fn(Vec<String>, ChunkingConfig, ProgressTicker) -> Iterable<TextChunk>;
