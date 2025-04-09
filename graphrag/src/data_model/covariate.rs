//! A package containing the 'Covariate' model.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A protocol for a covariate in the system.
///
/// Covariates are metadata associated with a subject, e.g. entity claims.
/// Each subject (e.g. entity) may be associated with multiple types of covariates.
#[derive(Serialize, Deserialize, Debug)]
pub struct Covariate {
    /// The ID of the item.
    pub id: String,

    /// Human readable ID used to refer to this community in prompts or texts displayed to users, such as in a report text (optional).
    #[serde(rename(deserialize = "human_readable_id"))]
    pub short_id: Option<String>,

    /// The subject id.
    pub subject_id: String,

    /// The subject type.
    pub subject_type: String,

    /// The covariate type.
    pub covariate_type: String,

    /// List of text unit IDs in which the covariate info appears (optional).
    pub text_unit_ids: Option<Vec<String>>,

    pub attributes: Option<HashMap<String, Value>>,
}
