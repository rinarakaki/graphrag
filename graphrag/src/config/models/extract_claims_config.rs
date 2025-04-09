//! Parameterization settings for the default configuration.

use std::collections::HashMap;

use crate::config::defaults::DEFAULT_CHAT_MODEL_ID;
// use crate::config::models::language_model_config::LanguageModelConfig;

/// Configuration section for claim extraction.
pub struct ClaimExtractionConfig {
    /// Whether claim extraction is enabled.
    pub enabled: bool,

    /// The model ID to use for claim extraction.
    pub model_id: String,

    /// The claim extraction prompt to use.
    pub prompt: Option<String>,

    /// The claim description to use.
    pub description: String,

    /// The maximum number of entity gleanings to use.
    pub max_gleanings: usize,

    /// The override strategy to use.
    pub strategy: Option<HashMap<String, String>>, // TODO(rinarakaki)

    /// The encoding model to use.
    pub encoding_model: Option<String>,
}

impl Default for ClaimExtractionConfig {
    /// Default values for claim extraction.
    fn default() -> Self {
        ClaimExtractionConfig {
            enabled: false,
            model_id: DEFAULT_CHAT_MODEL_ID.into(),
            prompt: None,
            description: "Any claims or facts that could be relevant to information discovery."
                .into(),
            max_gleanings: 1,
            strategy: None,
            encoding_model: None,
        }
    }
}

// impl ClaimExtractionConfig {
//     pub fn resolved_strategy(
//         self, root_dir: str, model_config: LanguageModelConfig
//     ) -> dict:
//         /// Get the resolved claim extraction strategy.
//         return self.strategy or {
//             "llm": model_config.model_dump(),
//             "num_threads": model_config.concurrent_requests,
//             "extraction_prompt": (Path(root_dir) / self.prompt).read_text(
//                 encoding="utf-8"
//             )
//             if self.prompt
//             else None,
//             "claim_description": self.description,
//             "max_gleanings": self.max_gleanings,
//             "encoding_name": model_config.encoding_model,
//         }
// }
