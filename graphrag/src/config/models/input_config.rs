//! Parameterization settings for the default configuration.

use std::collections::HashMap;

use crate::config::enums::{InputFileType, InputType};

/// The default configuration section for Input.
pub struct InputConfig {
    /// The input type to use.
    pub r#type: InputType,

    /// The input file type to use.
    pub file_type: InputFileType,

    /// The input base directory to use.
    pub base_dir: String,

    /// The azure blob storage connection string to use.
    pub connection_string: Option<String>,

    /// The storage account blob url to use.
    pub storage_account_blob_url: Option<String>,

    /// The azure blob storage container name to use.
    pub container_name: Option<String>,

    /// The input file encoding to use.
    pub encoding: String,

    /// The input file pattern to use.
    pub file_pattern: String,

    /// The optional file filter for the input files.
    pub file_filter: Option<HashMap<String, String>>,

    /// The input text column to use.
    pub text_column: String,

    /// The input title column to use.
    pub title_column: Option<String>,

    /// The document attribute columns to use.
    pub metadata: Option<Vec<String>>,
}

impl Default for InputConfig {
    /// Default values for input.
    fn default() -> Self {
        InputConfig {
            r#type: InputType::File,
            file_type: InputFileType::Text,
            base_dir: "input".into(),
            connection_string: None,
            storage_account_blob_url: None,
            container_name: None,
            encoding: "utf-8".into(),
            file_pattern: "".into(),
            file_filter: None,
            text_column: "text".into(),
            title_column: None,
            metadata: None,
        }
    }
}
