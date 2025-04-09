//! Parameterization settings for the default configuration.

use crate::config::defaults::DEFAULT_CHAT_MODEL_ID;

/// The default configuration section for Cache.
pub struct GlobalSearchConfig {
    /// The global search mapper prompt to use.
    pub map_prompt: Option<String>,

    /// The global search reducer to use.
    pub reduce_prompt: Option<String>,

    /// The model ID to use for global search.
    pub chat_model_id: String,

    /// The global search general prompt to use.
    pub knowledge_prompt: Option<String>,

    /// The temperature to use for token generation.
    pub temperature: f64,

    /// The top-p value to use for token generation.
    pub top_p: f64,

    /// The number of completions to generate.
    pub n: usize,

    /// The maximum context size in tokens.
    pub max_tokens: usize,

    /// The data llm maximum tokens.
    pub data_max_tokens: usize,

    /// The map llm maximum tokens.
    pub map_max_tokens: usize,

    /// The reduce llm maximum tokens.
    pub reduce_max_tokens: usize,

    /// The number of concurrent requests.
    pub concurrency: usize,

    // configurations for dynamic community selection
    /// LLM model to use for dynamic community selection
    pub dynamic_search_llm: String,

    /// Rating threshold in include a community report
    pub dynamic_search_threshold: usize,

    /// Keep parent community if any of the child communities are relevant
    pub dynamic_search_keep_parent: bool,

    /// Number of times to rate the same community report
    pub dynamic_search_num_repeats: usize,

    /// Use community summary instead of full_context
    pub dynamic_search_use_summary: bool,

    /// Number of concurrent coroutines to rate community reports
    pub dynamic_search_concurrent_coroutines: usize,

    /// The maximum level of community hierarchy to consider if none of the processed communities are relevant
    pub dynamic_search_max_level: usize,
}

impl Default for GlobalSearchConfig {
    /// Default values for global search.
    fn default() -> Self {
        GlobalSearchConfig {
            map_prompt: None,
            reduce_prompt: None,
            chat_model_id: DEFAULT_CHAT_MODEL_ID.into(),
            knowledge_prompt: None,
            temperature: 0.0,
            top_p: 1.0,
            n: 1,
            max_tokens: 12_000,
            data_max_tokens: 12_000,
            map_max_tokens: 1000,
            reduce_max_tokens: 2000,
            concurrency: 32,
            dynamic_search_llm: "gpt-4o-mini".into(),
            dynamic_search_threshold: 1,
            dynamic_search_keep_parent: false,
            dynamic_search_num_repeats: 1,
            dynamic_search_use_summary: false,
            dynamic_search_concurrent_coroutines: 16,
            dynamic_search_max_level: 2,
        }
    }
}
