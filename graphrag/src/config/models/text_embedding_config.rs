//! Parameterization settings for the default configuration.

use std::collections::HashMap;

use crate::config::defaults::{DEFAULT_EMBEDDING_MODEL_ID, DEFAULT_VECTOR_STORE_ID};
use crate::config::enums::TextEmbeddingTarget;
use crate::config::models::language_model_config::LanguageModelConfig;
use crate::index::operations::embed_text::TextEmbedStrategyType;

/// Configuration section for text embeddings.
pub struct TextEmbeddingConfig {
    /// The model ID to use for text embeddings.
    pub model_id: String,

    /// The vector store ID to use for text embeddings.
    pub vector_store_id: String,

    /// The batch size to use.
    pub batch_size: usize,

    /// The batch max tokens to use.
    pub batch_max_tokens: usize,

    /// The target to use. 'all', 'required', 'selected', or 'none'.
    pub target: TextEmbeddingTarget,

    /// The specific embeddings to perform.
    pub names: Vec<String>,

    /// The override strategy to use.
    pub strategy: Option<HashMap<String, String>>, // TODO(rinarakaki)
}

impl Default for TextEmbeddingConfig {
    /// Default values for embedding text.
    fn default() -> Self {
        TextEmbeddingConfig {
            model_id: DEFAULT_EMBEDDING_MODEL_ID.into(),
            vector_store_id: DEFAULT_VECTOR_STORE_ID.into(),
            batch_size: 16,
            batch_max_tokens: 8191,
            target: TextEmbeddingTarget::Required,
            names: Vec::new(),
            strategy: None,
            // model: str = "text-embedding-3-small"
        }
    }
}

impl TextEmbeddingConfig {
    /// Get the resolved text embedding strategy.
    pub fn resolved_strategy(self, model_config: LanguageModelConfig) -> HashMap<String, String> {
        self.strategy.unwrap_or(HashMap::from([
            ("type".to_string(), TextEmbedStrategyType::OpenAI),
            ("llm".to_string(), model_config.model_dump()),
            ("num_threads".to_string(), model_config.concurrent_requests),
            ("batch_size".to_string(), self.batch_size),
            ("batch_max_tokens".to_string(), self.batch_max_tokens),
        ]))
    }
}
