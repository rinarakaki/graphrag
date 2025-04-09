//! A module containing different lists and dictionaries.

use std::collections::HashMap;

pub type NodeList = Vec<&str>;
pub type EmbeddingList<T> = Vec<T>;
/// Label -> Embedding
pub type NodeEmbeddings = HashMap<String, Vec<f32>>;
