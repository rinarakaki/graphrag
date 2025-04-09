//! Parameterization settings for the default configuration.

use std::collections::HashMap;

use crate::config::defaults::DEFAULT_CHAT_MODEL_ID;
// use crate::config::models::language_model_config::LanguageModelConfig;

/// Configuration section for description summarization.
pub struct SummarizeDescriptionsConfig {
    /// The model ID to use for summarization.
    pub model_id: String,

    /// The description summarization prompt to use.
    pub prompt: Option<String>,

    /// The description summarization maximum length.
    pub max_length: usize,

    /// The override strategy to use.
    pub strategy: Option<HashMap<String, String>>,
}

impl Default for SummarizeDescriptionsConfig {
    /// Default values for summarizing descriptions.
    fn default() -> Self {
        SummarizeDescriptionsConfig {
            model_id: DEFAULT_CHAT_MODEL_ID.into(),
            prompt: None,
            max_length: 500,
            strategy: None,
        }
    }
}

// impl SummarizeDescriptionsConfig {
//     pub fn resolved_strategy(
//         self, root_dir: str, model_config: LanguageModelConfig
//     ) -> dict:
//         /// Get the resolved description summarization strategy.
//         use crate::index.operations.summarize_descriptions::(
//             SummarizeStrategyType,
//         )

//         return self.strategy or {
//             "type": SummarizeStrategyType.graph_intelligence,
//             "llm": model_config.model_dump(),
//             "num_threads": model_config.concurrent_requests,
//             "summarize_prompt": (Path(root_dir) / self.prompt).read_text(
//                 encoding="utf-8"
//             )
//             if self.prompt
//             else None,
//             "max_summary_length": self.max_length,
//         }
//     }
