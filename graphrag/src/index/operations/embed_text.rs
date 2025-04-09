//! The Indexing Engine text embed package root.

pub mod embed_text;
pub mod strategies;

pub use crate::index::operations::embed_text::embed_text::{
    TextEmbedStrategyType,
    embed_text,
};
