//! Parameterization settings for the default configuration.

use crate::config::enums::ReportingType;

/// The default configuration section for Reporting.
pub struct ReportingConfig {
    /// The reporting type to use.
    pub r#type: ReportingType,

    /// The base directory for reporting.
    pub base_dir: String,

    /// The reporting connection string to use.
    pub connection_string: Option<String>,

    /// The reporting container name to use.
    pub container_name: Option<String>,

    /// The storage account blob url to use.
    pub storage_account_blob_url: Option<String>,
}

impl Default for ReportingConfig {
    fn default() -> Self {
        ReportingConfig {
            r#type: ReportingType::File,
            base_dir: "logs".into(),
            connection_string: None,
            container_name: None,
            storage_account_blob_url: None,
        }
    }
}
