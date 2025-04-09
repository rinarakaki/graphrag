//! Parameterization settings for the default configuration.

pub use crate::config::enums::ChunkStrategyType;

/// Configuration section for chunking.
pub struct ChunkingConfig {
    /// The chunk size to use.
    pub size: usize,

    /// The chunk overlap to use.
    pub overlap: usize,

    /// The chunk by columns to use.
    pub group_by_columns: Vec<String>,

    /// The chunking strategy to use.
    pub strategy: ChunkStrategyType,

    /// The encoding model to use.
    pub encoding_model: String,

    /// Prepend metadata into each chunk.
    pub prepend_metadata: bool,

    /// Count metadata in max tokens.
    pub chunk_size_includes_metadata: bool,
}

impl Default for ChunkingConfig {
    /// Default values for chunks.
    fn default() -> Self {
        ChunkingConfig {
            size: 1200,
            overlap: 100,
            group_by_columns: vec!["id".into()],
            strategy: ChunkStrategyType::Tokens,
            encoding_model: "cl100k_base".into(),
            prepend_metadata: false,
            chunk_size_includes_metadata: false,
        }
    }
}
