//! A package containing the 'Identified' protocol.

/// A protocol for an item with an ID.
pub struct Identified {
    /// The ID of the item.
    pub id: String,

    /// Human readable ID used to refer to this community in prompts or texts displayed to users, such as in a report text (optional).
    pub short_id: Option<String>,
}
