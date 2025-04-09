//! A module containing load method definition.

use log::info;

use polars::prelude::LazyFrame;

use crate::config::models::input_config::InputConfig
use crate::index::input::util::load_files, process_data_columns
use crate::logger::base::ProgressLogger
use crate::storage::pipeline_storage::PipelineStorage

/// Load csv inputs from a directory.
pub async fn load_csv(
    config: InputConfig,
    progress: ProgressLogger | None,
    storage: PipelineStorage,
) -> LazyFrame {
    info!("Loading csv files from %s", config.base_dir);

    async fn load_file(path: str, group: dict | None) -> LazyFrame {
        if group is None:
            group = {}
        buffer = BytesIO(await storage.get(path, as_bytes=True))
        data = pd.read_csv(buffer, encoding=config.encoding)
        additional_keys = group.keys()
        if len(additional_keys) > 0:
            data[[*additional_keys]] = data.apply(
                lambda _row: pd.Series([group[key] for key in additional_keys]), axis=1
            )

        data = process_data_columns(data, config, path)

        creation_date = await storage.get_creation_date(path)
        data["creation_date"] = data.apply(lambda _: creation_date, axis=1)

        data
    }

    load_files(load_file, config, storage, progress).await
}
