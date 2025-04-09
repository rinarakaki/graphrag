//! Types for prompt tuning.

/// The type of document selection to use.
pub enum DocSelectionType {
    All,
    Random,
    Top,
    Auto,
}

impl DocSelectionType {
    /// Return the string representation of the enum value.
    pub fn as_str(&self) -> &str {
        match self {
            DocSelectionType::All => "all",
            DocSelectionType::Random => "random",
            DocSelectionType::Top => "top",
            DocSelectionType::Auto => "auto",
        }
    }
}
