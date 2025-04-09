//! Parameterization settings for the default configuration.

use crate::config::enums::OutputType;

/// The default configuration section for Output.
pub struct OutputConfig {
    /// The output type to use.
    pub r#type: OutputType,

    /// The base directory for the output.
    pub base_dir: String,

    /// The storage connection string to use.
    pub connection_string: Option<String>,

    /// The storage container name to use.
    pub container_name: Option<String>,

    /// The storage account blob url to use.
    pub storage_account_blob_url: Option<String>,

    /// The cosmosdb account url to use.
    pub cosmosdb_account_url: Option<String>,
}

impl Default for OutputConfig {
    /// Default values for output.
    fn default() -> Self {
        OutputConfig {
            r#type: OutputType::File,
            base_dir: "output".into(),
            connection_string: None,
            container_name: None,
            storage_account_blob_url: None,
            cosmosdb_account_url: None,
        }
    }
}
