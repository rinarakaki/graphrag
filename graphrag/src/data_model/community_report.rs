//! A package containing the 'CommunityReport' model.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

// (Named)
/// Defines an LLM-generated summary report of a community.
#[derive(Serialize, Deserialize, Debug)]
pub struct CommunityReport {
    /// The ID of the item.
    pub id: String,

    /// Human readable ID used to refer to this community in prompts or texts displayed to users, such as in a report text (optional).
    #[serde(rename(deserialize = "human_readable_id"))]
    pub short_id: Option<String>,

    /// The name/title of the item.
    pub title: String,

    /// The ID of the community this report is associated with.
    pub community_id: String,

    /// Summary of the report.
    pub summary: String, // = ""

    /// Full content of the report.
    pub full_content: String, // = ""

    /// Rank of the report, used for sorting (optional). Higher means more important
    pub rank: Option<f64>, // = 1.0

    /// The semantic (i.e. text) embedding of the full report content (optional).
    pub full_content_embedding: Option<Vec<f64>>, // = None

    /// A dictionary of additional attributes associated with the report (optional).
    pub attributes: Option<HashMap<String, Value>>, // = None

    /// The size of the report (Amount of text units).
    pub size: Option<usize>, // = None

    /// The period of the report (optional).
    pub period: Option<String>, // = None
}
