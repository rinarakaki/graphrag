//! Base classes for search algos.

use pandas as pd;
use tiktoken_rs;

use crate::language_model::protocol::base::ChatModel;
pub use crate::query::context_builder::builders::{
    BasicContextBuilder,
    DRIFTContextBuilder,
    GlobalContextBuilder,
    LocalContextBuilder,
};
use crate::query::context_builder::conversation_history::ConversationHistory;

/// A Structured Search Result.
pub struct SearchResult {
    response: str | HashMap<String, Box<dyn Any>> | list[HashMap<String, Box<dyn Any>>]
    context_data: str | Vec<LazyFrame> | dict[str, LazyFrame]
    // actual text strings that are in the context window, built from context_data
    context_text: str | Vec<str> | dict[str, str]
    completion_time: float
    // total LLM calls and token usage
    llm_calls: int
    prompt_tokens: int
    output_tokens: int
    // breakdown of LLM calls and token usage
    llm_calls_categories: dict[str, int] | None = None
    prompt_tokens_categories: dict[str, int] | None = None
    output_tokens_categories: dict[str, int] | None = None
}

/// The Base Search implementation.
pub trait BaseSearch<T> {
    fn new(
        self,
        model: ChatModel,
        context_builder: T,
        token_encoder: Option<tiktoken_rs::Encoding>,
        model_params: HashMap<String, Box<dyn Any>> | None = None,
        context_builder_params: HashMap<String, Box<dyn Any>> | None = None,
    ):
        self.model = model
        self.context_builder = context_builder
        self.token_encoder = token_encoder
        self.model_params = model_params or {}
        self.context_builder_params = context_builder_params or {}

    /// Search for the given query asynchronously.
    async fn search(
        &self,
        query: str,
        conversation_history: ConversationHistory | None = None,
        **kwargs,
    ) -> SearchResult;

    /// Stream search for the given query.
    async fn stream_search(
        &self,
        query: str,
        conversation_history: ConversationHistory | None = None,
    ) -> AsyncGenerator[str, None];
}
