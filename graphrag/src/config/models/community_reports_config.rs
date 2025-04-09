//! Parameterization settings for the default configuration.

use std::collections::HashMap;

use crate::config::defaults::DEFAULT_CHAT_MODEL_ID;
// use crate::config::models::language_model_config::LanguageModelConfig;

/// Configuration section for community reports.
pub struct CommunityReportsConfig {
    /// The model ID to use for community reports.
    pub model_id: String,

    /// The community report extraction prompt to use for graph-based summarization.
    pub graph_prompt: Option<String>,

    /// The community report extraction prompt to use for text-based summarization.
    pub text_prompt: Option<String>,

    /// The community report maximum length in tokens.
    pub max_length: usize,

    /// The maximum input length in tokens to use when generating reports.
    pub max_input_length: usize,

    /// The override strategy to use.
    pub strategy: Option<HashMap<String, String>>,
}

impl Default for CommunityReportsConfig {
    /// Default values for community report.
    fn default() -> Self {
        CommunityReportsConfig {
            model_id: DEFAULT_CHAT_MODEL_ID.into(),
            graph_prompt: None,
            text_prompt: None,
            max_length: 2000,
            max_input_length: 8000,
            strategy: None,
        }
    }
}

// impl CommunityReportsConfig {
//     pub fn resolved_strategy(
//         self, root_dir: str, model_config: LanguageModelConfig
//     ) -> dict:
//         /// Get the resolved community report extraction strategy.
//         use crate::index.operations.summarize_communities.typing::(
//             CreateCommunityReportsStrategyType,
//         )

//         return self.strategy or {
//             "type": CreateCommunityReportsStrategyType.graph_intelligence,
//             "llm": model_config.model_dump(),
//             "num_threads": model_config.concurrent_requests,
//             "graph_prompt": (Path(root_dir) / self.graph_prompt).read_text(
//                 encoding="utf-8"
//             )
//             if self.graph_prompt
//             else None,
//             "text_prompt": (Path(root_dir) / self.text_prompt).read_text(
//                 encoding="utf-8"
//             )
//             if self.text_prompt
//             else None,
//             "max_report_length": self.max_length,
//             "max_input_length": self.max_input_length,
//         }
// }
