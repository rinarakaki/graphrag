//! A package containing the 'Entity' model.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A protocol for an entity in the system.
#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    /// The ID of the item.
    pub id: String,

    /// Human readable ID used to refer to this community in prompts or texts displayed to users, such as in a report text (optional).
    #[serde(rename(deserialize = "human_readable_id"))]
    pub short_id: Option<String>,

    /// The name/title of the item.
    pub title: String,

    /// Type of the entity (can be any string, optional).
    pub r#type: Option<String>,

    /// Description of the entity (optional).
    pub description: Option<String>,

    /// The semantic (i.e. text) embedding of the entity (optional).
    pub description_embedding: Option<Vec<f64>>,

    /// The semantic (i.e. text) embedding of the entity (optional).
    pub name_embedding: Option<Vec<f64>>,

    /// The community IDs of the entity (optional).
    pub community_ids: Option<Vec<String>>,

    /// List of text unit IDs in which the entity appears (optional).
    pub text_unit_ids: Option<Vec<String>>,

    /// Rank of the entity, used for sorting (optional). Higher rank indicates more important entity. This can be based on centrality or other metrics.
    pub rank: Option<usize>, // TODO(rinarakaki) = 1,

    /// Additional attributes associated with the entity (optional), e.g. start time, end time, etc. To be included in the search prompt.
    pub attributes: Option<HashMap<String, Value>>,
}
