//! Base classes for global and local context builders.

use std::collections::HashMap;

use polars::prelude::LazyFrame;

use crate::query::context_builder::conversation_history::ConversationHistory;

/// A class to hold the results of the build_context.
pub struct ContextBuilderResult {
    pub context_chunks: Vec<String>,  // | str
    pub context_records: HashMap<String, LazyFrame>,
    pub llm_calls: usize, // = 0
    pub prompt_tokens: usize, // = 0
    pub output_tokens: usize, // = 0
}

/// Base class for global-search context builders.
pub trait GlobalContextBuilder {
    /// Build the context for the global search mode.
    pub async fn build_context(
        &self,
        query: str,
        conversation_history:  Option<ConversationHistory>,
        **kwargs,
    ) -> ContextBuilderResult;
}

/// Base class for local-search context builders.
pub trait LocalContextBuilder {
    /// Build the context for the local search mode.
    fn build_context(
        &self,
        query: str,
        conversation_history:  Option<ConversationHistory>,
        **kwargs,
    ) -> ContextBuilderResult;
}

/// Base class for DRIFT-search context builders.
pub trait DRIFTContextBuilder {
    /// Build the context for the primer search actions.
    async fn build_context(
        &self,
        query: str,
        **kwargs,
    ) -> (LazyFrame, HashMap<str, int>);
}

/// Base class for basic-search context builders.
pub trait BasicContextBuilder {
    /// Build the context for the basic search mode.
    fn build_context(
        &self,
        query: str,
        conversation_history:  Option<ConversationHistory>,
        **kwargs,
    ) -> ContextBuilderResult;
}
