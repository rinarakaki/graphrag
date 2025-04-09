//! Root package for description summarization.

pub mod summarize_descriptions;
pub mod typing;

pub use crate::index::operations::summarize_descriptions::summarize_descriptions::summarize_descriptions;
pub use crate::index::operations::summarize_descriptions::typing::{
    SummarizationStrategy,
    SummarizeStrategyType,
};
