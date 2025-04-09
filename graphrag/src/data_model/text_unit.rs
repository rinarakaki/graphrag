//! A package containing the 'TextUnit' model.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A protocol for a TextUnit item in a Document database.
#[derive(Serialize, Deserialize, Debug)]
pub struct TextUnit {
    /// The ID of the item.
    pub id: String,

    /// Human readable ID used to refer to this community in prompts or texts displayed to users, such as in a report text (optional).
    #[serde(rename(deserialize = "human_readable_id"))]
    pub short_id: Option<String>,

    /// The text of the unit.
    pub text: String,

    /// List of entity IDs related to the text unit (optional).
    pub entity_ids: Option<Vec<String>>,

    /// List of relationship IDs related to the text unit (optional).
    pub relationship_ids: Option<Vec<String>>,

    /// Dictionary of different types of covariates related to the text unit (optional).
    pub covariate_ids: Option<Vec<String>>,

    /// The number of tokens in the text (optional).
    pub n_tokens: Option<usize>,

    /// List of document IDs in which the text unit appears (optional).
    pub document_ids: Option<Vec<String>>,

    /// A dictionary of additional attributes associated with the text unit (optional).
    pub attributes: Option<HashMap<String, Value>>,
}
