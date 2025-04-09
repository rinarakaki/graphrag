//! Common default configuration values.

use crate::config::enums::{AuthType, ModelType};
use crate::config::models::graph_rag_config::GraphRagConfig;
use crate::config::models::language_model_config::LanguageModelConfig;
use crate::config::models::vector_store_config::VectorStoreConfig;

pub const DEFAULT_OUTPUT_BASE_DIR: &str = "output";
pub const DEFAULT_CHAT_MODEL_ID: &str = "default_chat_model";
pub const DEFAULT_CHAT_MODEL_TYPE: ModelType = ModelType::OpenAIChat;
pub const DEFAULT_CHAT_MODEL: &str = "gpt-4-turbo-preview";
pub const DEFAULT_CHAT_MODEL_AUTH_TYPE: AuthType = AuthType::APIKey;
pub const DEFAULT_EMBEDDING_MODEL_ID: &str = "default_embedding_model";
pub const DEFAULT_EMBEDDING_MODEL_TYPE: ModelType = ModelType::OpenAIEmbedding;
pub const DEFAULT_EMBEDDING_MODEL: &str = "text-embedding-3-small";
pub const DEFAULT_EMBEDDING_MODEL_AUTH_TYPE: AuthType = AuthType::APIKey;
pub const DEFAULT_VECTOR_STORE_ID: &str = "default_vector_store";

pub const ENCODING_MODEL: &str = "cl100k_base";
pub const COGNITIVE_SERVICES_AUDIENCE: &str = "https://cognitiveservices.azure.com/.default";

pub const GRAPHRAG_CONFIG: GraphRagConfig = GraphRagConfig::default();
pub const LANGUAGE_MODEL: LanguageModelConfig = LanguageModelConfig::default();
pub const VECTOR_STORE: VectorStoreConfig = VectorStoreConfig::default();
