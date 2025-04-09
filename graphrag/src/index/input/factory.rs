//! A module containing create_input method definition.

use log;

use std::path::Path;

use polars::prelude::LazyFrame;

use crate::config::enums::{InputFileType, InputType};
use crate::config::models::input_config::InputConfig;
use crate::index::input::csv::load_csv;
use crate::index::input::json::load_json;
use crate::index::input::text::load_text;
use crate::logger::base::ProgressLogger;
use crate::logger::null_progress::NullProgressLogger;
use crate::storage::blob_pipeline_storage::BlobPipelineStorage;
use crate::storage::file_pipeline_storage::FilePipelineStorage;

loaders: dict[str, Callable[..., Awaitable[LazyFrame]]] = {
    InputFileType.text: load_text,
    InputFileType.csv: load_csv,
    InputFileType.json: load_json,
}

/// Instantiate input data for a pipeline.
pub async fn create_input(
    config: InputConfig,
    progress_reporter: ProgressLogger | None = None,
    root_dir: str | None = None,
) -> LazyFrame {
    let root_dir = root_dir or ""
    info!("loading input from root_dir=%s", config.base_dir)
    progress_reporter = progress_reporter or NullProgressLogger()

    match config.r#type {
        InputType::Blob => {
            info!("using blob storage input")
            if config.container_name is None:
                msg = "Container name required for blob storage"
                raise ValueError(msg)
            if (
                config.connection_string is None
                and config.storage_account_blob_url is None
            ):
                msg = "Connection string or storage account blob url required for blob storage"
                raise ValueError(msg)
            storage = BlobPipelineStorage(
                connection_string=config.connection_string,
                storage_account_blob_url=config.storage_account_blob_url,
                container_name=config.container_name,
                path_prefix=config.base_dir,
            )
        }
        InputType::File {
            info!("using file storage for input")
            storage = FilePipelineStorage(
                root_dir=str(Path(root_dir) / (config.base_dir or ""))
            )
        }
    }

    if config.file_type in loaders {
        progress = progress_reporter.child(
            f"Loading Input ({config.file_type})", transient=False
        )
        loader = loaders[config.file_type]
        result = loader(config, progress, storage).await;
        // Convert metadata columns to strings and collapse them into a JSON object
        if config.metadata:
            if all(col in result.columns for col in config.metadata):
                // Collapse the metadata columns into a single JSON object column
                result["metadata"] = result[config.metadata].apply(
                    lambda row: row.to_dict(), axis=1
                )
            else:
                value_error_msg = (
                    "One or more metadata columns not found in the DataFrame."
                )
                raise ValueError(value_error_msg)

            result[config.metadata] = result[config.metadata].astype(str)

        return cast("LazyFrame", result)
    }

    msg = f"Unknown input type {config.file_type}"
    raise ValueError(msg)
}
