//! A package containing a factory for supported llm types.

use crate::config::enums::ModelType;
use crate::language_model::protocol::{ChatModel, EmbeddingModel};
use crate::language_model::providers::fnllm::models::{
    AzureOpenAIChatFNLLM,
    AzureOpenAIEmbeddingFNLLM,
    OpenAIChatFNLLM,
    OpenAIEmbeddingFNLLM,
};

/// A factory for creating Model instances.
pub struct ModelFactory {
    _chat_registry: ClassVar[dict[str, Callable[..., ChatModel]]] = {}
    _embedding_registry: ClassVar[dict[str, Callable[..., EmbeddingModel]]] = {}

    /// Register a ChatModel implementation.
    pub fn register_chat(cls, model_type: str, creator: Callable[..., ChatModel]) {
        cls._chat_registry[model_type] = creator
    }

    /// Register an EmbeddingModel implementation.
    pub fn register_embedding(
        cls, model_type: str, creator: Callable[..., EmbeddingModel]
    ) {
        cls._embedding_registry[model_type] = creator
    }

    /**
    Create a ChatModel instance.

    Args:
        model_type: The type of ChatModel to create.
        **kwargs: Additional keyword arguments for the ChatModel constructor.

    Returns
    -------
        A ChatModel instance.
     */
    pub fn create_chat_model(cls, model_type: str, **kwargs: Any) -> ChatModel {
        if model_type not in cls._chat_registry {
            msg = format!("ChatMOdel implementation '{model_type}' is not registered.")
            raise ValueError(msg)
        }
        cls._chat_registry[model_type](**kwargs)
    }

    /**
    Create an EmbeddingModel instance.

    Args:
        model_type: The type of EmbeddingModel to create.
        **kwargs: Additional keyword arguments for the EmbeddingLLM constructor.

    Returns
    -------
        An EmbeddingLLM instance.
    */
    pub fn create_embedding_model(cls, model_type: str, **kwargs: Any) -> EmbeddingModel {
        if model_type not in cls._embedding_registry {
            msg = format!("EmbeddingModel implementation '{model_type}' is not registered.")
            raise ValueError(msg)
        }
        cls._embedding_registry[model_type](**kwargs)
    }

    /// Get the registered ChatModel implementations.
    pub fn get_chat_models(cls) -> Vec<String> {
        list(cls._chat_registry.keys())
    }

    /// Get the registered EmbeddingModel implementations.
    pub fn get_embedding_models(cls) -> Vec<String>:{
        return list(cls._embedding_registry.keys())
    }

    /// Check if the given model type is supported.
    pub fn is_supported_chat_model(cls, model_type: str) -> bool {
        return model_type in cls._chat_registry
    }

    /// Check if the given model type is supported.
    pub fn is_supported_embedding_model(cls, model_type: str) -> bool {
        model_type in cls._embedding_registry
    }

        /// Check if the given model type is supported.
    pub fn is_supported_model(cls, model_type: str) -> bool {
        cls.is_supported_chat_model(
            model_type
        ) || cls.is_supported_embedding_model(model_type)
    }
}

// --- Register default implementations ---
ModelFactory::register_chat(
    ModelType::AzureOpenAIChat, lambda **kwargs: AzureOpenAIChatFNLLM(**kwargs)
)
ModelFactory::register_chat(
    ModelType::OpenAIChat, lambda **kwargs: OpenAIChatFNLLM(**kwargs)
)

ModelFactory::register_embedding(
    ModelType::AzureOpenAIEmbedding, lambda **kwargs: AzureOpenAIEmbeddingFNLLM(**kwargs)
)
ModelFactory::register_embedding(
    ModelType::OpenAIEmbedding, lambda **kwargs: OpenAIEmbeddingFNLLM(**kwargs)
)
