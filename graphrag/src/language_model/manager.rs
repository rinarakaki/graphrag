/*!
Singleton LLM Manager for ChatLLM and EmbeddingsLLM instances.

This manager lets you register chat and embeddings LLMs independently.
It leverages the LLMFactory for instantiation.
*/

use crate::language_model::factory::ModelFactory;
use crate::language_model::protocol::{ChatModel, EmbeddingModel};

/// Singleton manager for LLM instances.
pub struct ModelManager {
    _instance: ClassVar[ModelManager | None] = None
}

impl ModelManager {
    def __new__(cls) -> ModelManager:
        /// Create a new instance of LLMManager if it does not exist.
        if cls._instance.is_none():
            cls._instance = super().__new__(cls)
        return cls._instance

    pub fn new() -> Self {
        // Avoid reinitialization in the singleton.
        if not hasattr(self, "_initialized") {
            self.chat_models: dict[str, ChatModel] = {}
            self.embedding_models: dict[str, EmbeddingModel] = {}
            self._initialized = True
        }
    }

    @classmethod
    /// Return the singleton instance of LLMManager.
    pub fn get_instance(cls) -> ModelManager {
        return cls.__new__(cls)
    }

    /**
    Register a ChatLLM instance under a unique name.

    Args:
        name: Unique identifier for the ChatLLM instance.
        model_type: Key for the ChatLLM implementation in LLMFactory.
        **chat_kwargs: Additional parameters for instantiation.
     */
    pub fn register_chat(
        self, name: str, model_type: str, **chat_kwargs: Any
    ) -> ChatModel {
        chat_kwargs["name"] = name
        self.chat_models[name] = ModelFactory.create_chat_model(
            model_type, **chat_kwargs
        )
        return self.chat_models[name]
    }

    /**
    Register an EmbeddingsLLM instance under a unique name.

    Args:
        name: Unique identifier for the EmbeddingsLLM instance.
        embedding_key: Key for the EmbeddingsLLM implementation in LLMFactory.
        **embedding_kwargs: Additional parameters for instantiation.
     */
    pub fn register_embedding(
        self, name: str, model_type: str, **embedding_kwargs: Any
    ) -> EmbeddingModel {
        embedding_kwargs["name"] = name
        self.embedding_models[name] = ModelFactory.create_embedding_model(
            model_type, **embedding_kwargs
        )
        return self.embedding_models[name]
    }

    /**
    Retrieve the ChatLLM instance registered under the given name.

    Raises
    ------
        ValueError: If no ChatLLM is registered under the name.
     */
    pub fn get_chat_model(self, name: str) -> ChatModel | None {
        if name not in self.chat_models:
            msg = format!("No ChatLLM registered under the name '{name}'.")
            raise ValueError(msg)
        return self.chat_models[name]
    }

    /**
    Retrieve the EmbeddingsLLM instance registered under the given name.

    Raises
    ------
        ValueError: If no EmbeddingsLLM is registered under the name.
     */
    pub fn get_embedding_model(self, name: str) -> EmbeddingModel | None {
        if name not in self.embedding_models:
            msg = format!("No EmbeddingsLLM registered under the name '{name}'.")
            raise ValueError(msg)
        return self.embedding_models[name]
    }

    /**
    Retrieve the ChatLLM instance registered under the given name.

    If the ChatLLM does not exist, it is created and registered.

    Args:
        name: Unique identifier for the ChatLLM instance.
        model_type: Key for the ChatModel implementation in LLMFactory.
        **chat_kwargs: Additional parameters for instantiation.
     */
    pub fn get_or_create_chat_model(
        self, name: str, model_type: str, **chat_kwargs: Any
    ) -> ChatModel {
        if name not in self.chat_models:
            return self.register_chat(name, model_type, **chat_kwargs)
        return self.chat_models[name]
    }

    /**
    Retrieve the EmbeddingsLLM instance registered under the given name.

    If the EmbeddingsLLM does not exist, it is created and registered.

    Args:
        name: Unique identifier for the EmbeddingsLLM instance.
        model_type: Key for the EmbeddingsLLM implementation in LLMFactory.
        **embedding_kwargs: Additional parameters for instantiation.
     */
    pub fn get_or_create_embedding_model(
        &self, name: &str, model_type: &str, **embedding_kwargs: Any
    ) -> EmbeddingModel {
        if name not in self.embedding_models:
            return self.register_embedding(name, model_type, **embedding_kwargs)
        return self.embedding_models[name]
    }

    /// Remove the ChatLLM instance registered under the given name.
    pub fn remove_chat(self, name: str) {
        self.chat_models.pop(name, None)
    }

    /// Remove the EmbeddingsLLM instance registered under the given name.
    pub fn remove_embedding(self, name: str) {
        self.embedding_models.pop(name, None)
    }

    /// Return a copy of all registered ChatLLM instances.
    pub fn list_chat_models(self) -> dict[str, ChatModel] {
        return dict(self.chat_models)
    }

    /// Return a copy of all registered EmbeddingsLLM instances.
    pub fn list_embedding_models(self) -> dict[str, EmbeddingModel] {
        return dict(self.embedding_models)
    }
}
