//! A module containing the pipeline reporter factory.

use std::path::Path;

use crate::callbacks::blob_workflow_callbacks::BlobWorkflowCallbacks;
use crate::callbacks::console_workflow_callbacks::ConsoleWorkflowCallbacks;
use crate::callbacks::file_workflow_callbacks::FileWorkflowCallbacks;
use crate::callbacks::workflow_callbacks::WorkflowCallbacks;
use crate::config::enums::ReportingType;
use crate::config::models::reporting_config::ReportingConfig;

/// Create a logger for the given pipeline config.
pub fn create_pipeline_reporter(
    config: Option<ReportingConfig>,
    root_dir: Option<String>,
) -> Box<dyn WorkflowCallbacks> {
    let config = config.unwrap_or(ReportingConfig {
        r#type: ReportingType::File,
        base_dir: Some("logs".into()),
        connection_string: None,
        container_name: None,
        storage_account_blob_url: None,
    });
    match config.r#type {
        ReportingType::File => Box::new(FileWorkflowCallbacks::new(
            &Path::new(&root_dir.unwrap_or_default())
                .join(config.base_dir.unwrap_or_default())
                .to_string_lossy()
                .to_string(),
        )),
        ReportingType::Console => Box::new(ConsoleWorkflowCallbacks),
        ReportingType::Blob => Box::new(BlobWorkflowCallbacks::new(
            config.connection_string,
            config.container_name,
            "",
            Some(config.base_dir),
            config.storage_account_blob_url,
        )),
    }
}
