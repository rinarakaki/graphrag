//! Parameterization settings for the default configuration.

use std::collections::HashMap;
use std::path::Path;

use crate::config::defaults::DEFAULT_CHAT_MODEL_ID;
use crate::config::models::language_model_config::LanguageModelConfig;
use crate::index::operations::extract_graph::typing::ExtractEntityStrategyType;

/// Configuration section for entity extraction.
pub struct ExtractGraphConfig {
    /// The model ID to use for text embeddings.
    pub model_id: String,

    /// The entity extraction prompt to use.
    pub prompt: Option<String>,

    /// The entity extraction entity types to use.
    pub entity_types: Vec<String>,

    /// The maximum number of entity gleanings to use.
    pub max_gleanings: usize,

    /// Override the default entity extraction strategy
    pub strategy: Option<HashMap<String, String>>,

    /// The encoding model to use.
    pub encoding_model: Option<String>,
}

impl Default for ExtractGraphConfig {
    /// Default values for extracting graph.
    fn default() -> Self {
        ExtractGraphConfig {
            model_id: DEFAULT_CHAT_MODEL_ID.into(),
            prompt: None,
            entity_types: vec![
                "organization".into(),
                "person".into(),
                "geo".into(),
                "event".into(),
            ],
            max_gleanings: 1,
            strategy: None,
            encoding_model: None,
        }
    }
}

impl ExtractGraphConfig {
    /// Get the resolved entity extraction strategy.
    pub fn resolved_strategy(self, root_dir: &str, model_config: LanguageModelConfig) -> HashMap {
        self.strategy.unwrap_or(HashMap::from([
            ("type", ExtractEntityStrategyType.graph_intelligence),
            ("llm", model_config.model_dump()),
            ("num_threads", model_config.concurrent_requests),
            (
                "extraction_prompt",
                match self.prompt {
                    Some(prompt) => (Path(root_dir) / self.prompt).read_text(encoding = "utf-8"),
                    None => None,
                },
            ),
            ("max_gleanings", self.max_gleanings),
            ("encoding_name", model_config.encoding_model),
        ]))
    }
}
