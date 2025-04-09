//! A package containing the 'Community' model.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A protocol for a community in the system.
#[derive(Serialize, Deserialize, Debug)]
pub struct Community {
    /// The ID of the item.
    pub id: String,

    /// Human readable ID used to refer to this community in prompts or texts displayed to users, such as in a report text (optional).
    #[serde(rename(deserialize = "human_readable_id"))]
    pub short_id: Option<String>,

    /// The name/title of the item.
    pub title: String,

    /// Community level.
    pub level: String,

    /// Community ID of the parent node of this community.
    pub parent: String,

    /// List of community IDs of the child nodes of this community.
    pub children: Vec<String>,

    /// List of entity IDs related to the community (optional).
    #[serde(default)]
    pub entity_ids: Option<Vec<String>>,

    /// List of relationship IDs related to the community (optional).
    #[serde(default)]
    pub relationship_ids: Option<Vec<String>>,

    /// Dictionary of different types of covariates related to the community (optional), e.g. claims
    #[serde(default)]
    pub covariate_ids: Option<HashMap<String, Vec<String>>>,

    /// A dictionary of additional attributes associated with the community (optional). To be included in the search prompt.
    #[serde(default)]
    pub attributes: Option<HashMap<String, Value>>,

    /// The size of the community (Amount of text units).
    #[serde(default)]
    pub size: Option<usize>,

    #[serde(default)]
    pub period: Option<String>,
}
