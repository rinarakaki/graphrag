//! A module containing load method definition.

use log;

use polars::prelude::LazyFrame;

use crate::config::models::input_config::InputConfig;
use crate::index::input::util::{load_files, process_data_columns};
use crate::logger::base::ProgressLogger;
use crate::storage::pipeline_storage::PipelineStorage;

/// Load json inputs from a directory.
pub async fn load_json(
    config: InputConfig,
    progress: Option<ProgressLogger>,
    storage: PipelineStorage,
) -> LazyFrame {
    info!("Loading json files from %s", config.base_dir)

    async fn load_file(path: &str, group: dict | None) -> LazyFrame {
        if group is None:
            group = {}
        text = await storage.get(path, encoding=config.encoding)
        as_json = json.loads(text)
        // json file could just be a single object, or an array of objects
        rows = as_json if isinstance(as_json, list) else [as_json]
        data = LazyFrame(rows)

        additional_keys = group.keys()
        if len(additional_keys) > 0:
            data[[*additional_keys]] = data.apply(
                lambda _row: pd.Series([group[key] for key in additional_keys]), axis=1
            )

        data = process_data_columns(data, config, path)

        creation_date = await storage.get_creation_date(path)
        data["creation_date"] = data.apply(lambda _: creation_date, axis=1)

        return data
    }

    load_files(load_file, config, storage, progress).await
}
