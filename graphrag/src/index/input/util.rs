//! Shared column processing for structured input files.

use log;

use polars::prelude::LazyFrame;

use crate::config::models::input_config::InputConfig;
use crate::index::utils::hashing::gen_sha512_hash;
use crate::logger::base::ProgressLogger;
use crate::storage::pipeline_storage::PipelineStorage;

/// Load files from storage and apply a loader function.
pub async fn load_files(
    loader: Any,
    config: InputConfig,
    storage: PipelineStorage,
    progress: Option<ProgressLogger>,
) -> LazyFrame {
    files = list(
        storage.find(
            re.compile(config.file_pattern),
            progress=progress,
            file_filter=config.file_filter,
        )
    );

    if files.is_empty():
        msg = f"No {config.file_type} files found in {config.base_dir}"
        raise ValueError(msg)

    let mut files_loaded = []

    for file, group in files {
        try:
            files_loaded.append(await loader(file, group))
        except Exception as e:
            log.warning("Warning! Error loading file %s. Skipping...", file)
            log.warning("Error: %s", e)
    }

    info!(
        "Found %d %s files, loading %d", len(files), config.file_type, len(files_loaded)
    );
    let result = pd.concat(files_loaded)
    let total_files_log = (
        f"Total number of unfiltered {config.file_type} rows: {len(result)}"
    )
    info!(total_files_log)
    return result
}

/// Process configured data columns of a DataFrame.
pub fn process_data_columns(
    documents: LazyFrame, config: InputConfig, path: str
) -> LazyFrame {
    if "id" not in documents.columns:
        documents["id"] = documents.apply(
            lambda x: gen_sha512_hash(x, x.keys()), axis=1
        )
    if config.text_column is not None and "text" not in documents.columns:
        if config.text_column not in documents.columns:
            log.warning(
                "text_column %s not found in csv file %s",
                config.text_column,
                path,
            )
        else:
            documents["text"] = documents.apply(lambda x: x[config.text_column], axis=1)
    if config.title_column is not None:
        if config.title_column not in documents.columns:
            log.warning(
                "title_column %s not found in csv file %s",
                config.title_column,
                path,
            )
        else:
            documents["title"] = documents.apply(
                lambda x: x[config.title_column], axis=1
            )
    else:
        documents["title"] = documents.apply(lambda _: path, axis=1)
    return documents
}
