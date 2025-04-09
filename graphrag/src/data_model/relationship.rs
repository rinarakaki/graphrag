//! A package containing the 'Relationship' model."""

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A relationship between two entities. This is a generic relationship, and can be used to represent any type of relationship between any two entities.
#[derive(Serialize, Deserialize, Debug)]
pub struct Relationship {
    /// The ID of the item.
    pub id: String,

    /// Human readable ID used to refer to this community in prompts or texts displayed to users, such as in a report text (optional).
    #[serde(rename(deserialize = "human_readable_id"))]
    pub short_id: Option<String>,

    /// The source entity name.
    pub source: String,

    /// The target entity name.
    pub target: String,

    /// The edge weight.
    pub weight: Option<f64>, // = 1.0,

    /// A description of the relationship (optional).
    pub description: Option<String>, // = None,

    /// The semantic embedding for the relationship description (optional).
    pub description_embedding: Option<Vec<f64>>, // = None,

    /// List of text unit IDs in which the relationship appears (optional).
    pub text_unit_ids: Option<Vec<String>>, // = None,

    /// Rank of the relationship, used for sorting (optional). Higher rank indicates more important relationship. This can be based on centrality or other metrics.
    pub rank: Option<usize>, // = 1,

    /// Additional attributes associated with the relationship (optional). To be included in the search prompt
    pub attributes: Option<HashMap<String, Value>>,
}
