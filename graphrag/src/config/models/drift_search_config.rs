//! Parameterization settings for the default configuration.

use crate::config::defaults::{DEFAULT_CHAT_MODEL_ID, DEFAULT_EMBEDDING_MODEL_ID};

/// The default configuration section for Cache.
pub struct DRIFTSearchConfig {
    /// The drift search prompt to use.
    pub prompt: Option<String>,

    /// The drift search reduce prompt to use.
    pub reduce_prompt: Option<String>,

    /// The model ID to use for drift search.
    pub chat_model_id: String,

    /// The model ID to use for drift search.
    pub embedding_model_id: String,

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

    /// The reduce llm maximum tokens response to produce.
    pub reduce_max_tokens: usize,

    /// The temperature to use for token generation in reduce.
    pub reduce_temperature: f64,

    /// The number of concurrent requests.
    pub concurrency: usize,

    /// The number of top global results to retrieve.
    pub drift_k_followups: usize,

    /// The number of folds for search priming.
    pub primer_folds: usize,

    /// The maximum number of tokens for the LLM in primer.
    pub primer_llm_max_tokens: usize,

    /// The number of drift search steps to take.
    pub n_depth: usize,

    /// The proportion of search dedicated to text units.
    pub local_search_text_unit_prop: f64,

    /// The proportion of search dedicated to community properties.
    pub local_search_community_prop: f64,

    /// The number of top K entities to map during local search.
    pub local_search_top_k_mapped_entities: usize,

    /// The number of top K relationships to map during local search.
    pub local_search_top_k_relationships: usize,

    /// The maximum context size in tokens for local search.
    pub local_search_max_data_tokens: usize,

    /// The temperature to use for token generation in local search.
    pub local_search_temperature: f64,

    /// The top-p value to use for token generation in local search.
    pub local_search_top_p: f64,

    /// The number of completions to generate in local search.
    pub local_search_n: usize,

    /// The maximum number of generated tokens for the LLM in local search.
    pub local_search_llm_max_gen_tokens: usize,
}

impl Default for DRIFTSearchConfig {
    /// Default values for drift search.
    fn default() -> Self {
        DRIFTSearchConfig {
            prompt: None,
            reduce_prompt: None,
            chat_model_id: DEFAULT_CHAT_MODEL_ID.into(),
            embedding_model_id: DEFAULT_EMBEDDING_MODEL_ID.into(),
            temperature: 0.0,
            top_p: 1.0,
            n: 1,
            max_tokens: 12_000,
            data_max_tokens: 12_000,
            reduce_max_tokens: 2_000,
            reduce_temperature: 0.0,
            concurrency: 32,
            drift_k_followups: 20,
            primer_folds: 5,
            primer_llm_max_tokens: 12_000,
            n_depth: 3,
            local_search_text_unit_prop: 0.9,
            local_search_community_prop: 0.1,
            local_search_top_k_mapped_entities: 10,
            local_search_top_k_relationships: 10,
            local_search_max_data_tokens: 12_000,
            local_search_temperature: 0.0,
            local_search_top_p: 1.0,
            local_search_n: 1,
            local_search_llm_max_gen_tokens: 4_096,
        }
    }
}
