//! Base llm protocol definitions.

use crate::language_model::response::base::ModelResponse;

/**
Protocol for an embedding-based Language Model (LM).

This protocol defines the methods required for an embedding-based LM.
 */
pub trait EmbeddingModel {
    /**
    Generate an embedding vector for the given list of strings.

    Args:
        text: The text to generate an embedding for.
        **kwargs: Additional keyword arguments (e.g., model parameters).

    Returns
    -------
        A collections of list of floats representing the embedding vector for each item in the batch.
     */
    async fn aembed_batch(
        self, text_list: Vec<String>, **kwargs: Any
    ) -> Vec<Vec<f64>>;

    /**
    Generate an embedding vector for the given text.

    Args:
        text: The text to generate an embedding for.
        **kwargs: Additional keyword arguments (e.g., model parameters).

    Returns
    -------
        A list of floats representing the embedding vector.
     */
    async fn aembed(self, text: str, **kwargs: Any) -> Vec<f64>;

    /**
    Generate an embedding vector for the given list of strings.

    Args:
        text: The text to generate an embedding for.
        **kwargs: Additional keyword arguments (e.g., model parameters).

    Returns
    -------
        A collections of list of floats representing the embedding vector for each item in the batch.
    */
    fn embed_batch(self, text_list: Vec<String>, **kwargs: Any) -> Vec<Vec<f64>>;

    /**
    Generate an embedding vector for the given text.

    Args:
        text: The text to generate an embedding for.
        **kwargs: Additional keyword arguments (e.g., model parameters).

    Returns
    -------
        A list of floats representing the embedding vector.
     */
    fn embed(self, text: str, **kwargs: Any) -> Vec<f64>;
}

/**
Protocol for a chat-based Language Model (LM).

This protocol defines the methods required for a chat-based LM.
Prompt is always required for the chat method, and any other keyword arguments are forwarded to the Model provider.
 */
pub trait ChatModel {
    /**
    Generate a response for the given text.

    Args:
        prompt: The text to generate a response for.
        history: The conversation history.
        **kwargs: Additional keyword arguments (e.g., model parameters).

    Returns
    -------
        A string representing the response.

     */
    async fn achat(
        &self, prompt: str, history: list | None = None, **kwargs: Any
    ) -> ModelResponse;

    /**
    Generate a response for the given text using a streaming interface.

    Args:
        prompt: The text to generate a response for.
        history: The conversation history.
        **kwargs: Additional keyword arguments (e.g., model parameters).

    Returns
    -------
        A generator that yields strings representing the response.
     */
    async fn achat_stream(
        &self, prompt: str, history: list | None = None, **kwargs: Any
    ) -> AsyncGenerator[str, None] {
        yield ""  // Yield an empty string so that the function is recognized as a generator
    }

    /**
    Generate a response for the given text.

    Args:
        prompt: The text to generate a response for.
        history: The conversation history.
        **kwargs: Additional keyword arguments (e.g., model parameters).

    Returns
    -------
        A string representing the response.

    */
    fn chat(
        &self, prompt: str, history: list | None = None, **kwargs: Any
    ) -> ModelResponse;

    /**
    Generate a response for the given text using a streaming interface.

    Args:
        prompt: The text to generate a response for.
        history: The conversation history.
        **kwargs: Additional keyword arguments (e.g., model parameters).

    Returns
    -------
        A generator that yields strings representing the response.
     */
    fn chat_stream(
        &self, prompt: str, history: list | None = None, **kwargs: Any
    ) -> Generator[str, None];
}
