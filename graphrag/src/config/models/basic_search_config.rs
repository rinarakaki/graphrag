//! Parameterization settings for the default configuration.

use crate::config::defaults::{DEFAULT_CHAT_MODEL_ID, DEFAULT_EMBEDDING_MODEL_ID};

/// The default configuration section for Cache.
pub struct BasicSearchConfig {
    /// The basic search prompt to use.
    pub prompt: Option<String>,

    /// The model ID to use for basic search.
    pub chat_model_id: String,

    /// The model ID to use for text embeddings.
    pub embedding_model_id: String,

    /// The text unit proportion.
    pub text_unit_prop: f64,

    /// The conversation history maximum turns.
    pub conversation_history_max_turns: usize,

    /// The temperature to use for token generation.
    pub temperature: f64,

    /// The top-p value to use for token generation.
    pub top_p: f64,

    /// The number of completions to generate.
    pub n: usize,

    /// The maximum tokens.
    pub max_tokens: usize,

    /// The LLM maximum tokens.
    pub llm_max_tokens: usize,
}

impl Default for BasicSearchConfig {
    /// Default values for basic search.
    fn default() -> Self {
        BasicSearchConfig {
            prompt: None,
            chat_model_id: DEFAULT_CHAT_MODEL_ID.into(),
            embedding_model_id: DEFAULT_EMBEDDING_MODEL_ID.into(),
            text_unit_prop: 0.5,
            conversation_history_max_turns: 5,
            temperature: 0.0,
            top_p: 1.0,
            n: 1,
            max_tokens: 12_000,
            llm_max_tokens: 2000,
        }
    }
}
