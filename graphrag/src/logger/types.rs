//! Logging types.
//!
//! This module defines the types of loggers that can be used.

// Note: Code in this module was not included in the factory module because it negatively impacts the CLI experience.
/// The type of logger to use.
pub enum LoggerType {
    Rich,
    Print,
    None,
}

impl LoggerType {
    /// Return a string representation of the enum value.
    pub fn as_str(&self) -> &str {
        match self {
            LoggerType::Rich => "rich",
            LoggerType::Print => "print",
            LoggerType::None => "none",
        }
    }
}
