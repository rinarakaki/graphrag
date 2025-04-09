//! Common types for the GraphRAG knowledge model.

pub type TextEmbedder = fn(&str) -> Vec<f64>;
