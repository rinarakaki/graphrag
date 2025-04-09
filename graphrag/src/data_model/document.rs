//! A package containing the 'Document' model.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A protocol for a document in the system.
#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    /// The ID of the item.
    pub id: String,

    /// Human readable ID used to refer to this community in prompts or texts displayed to users, such as in a report text (optional).
    #[serde(rename(deserialize = "human_readable_id"))]
    pub short_id: Option<String>,

    /// The name/title of the item.
    pub title: String,

    /// Type of the document.
    pub r#type: String,

    /// list of text units in the document.
    pub text_unit_ids: Vec<String>,

    /// The raw text content of the document.
    pub text: String,

    /// A dictionary of structured attributes such as author, etc (optional).
    pub attributes: Option<HashMap<String, Value>>,
}
