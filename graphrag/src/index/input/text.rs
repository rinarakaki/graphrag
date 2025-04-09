//! A module containing load method definition.

use std::path::Path;

use log;
use polars::prelude::LazyFrame;

use crate::config::models::input_config::InputConfig
use crate::index::input::util::load_files
use crate::index::utils::hashing::gen_sha512_hash
use crate::logger::base::ProgressLogger
use crate::storage::pipeline_storage::PipelineStorage

/// Load text inputs from a directory.
pub async fn load_text(
    config: InputConfig,
    progress: ProgressLogger | None,
    storage: PipelineStorage,
) -> LazyFrame {
    async def load_file(path: str, group: dict | None = None) -> LazyFrame {
        if group is None:
            group = {}
        text = await storage.get(path, encoding=config.encoding)
        new_item = {**group, "text": text}
        new_item["id"] = gen_sha512_hash(new_item, new_item.keys())
        new_item["title"] = str(Path(path).name)
        new_item["creation_date"] = await storage.get_creation_date(path)
        return LazyFrame([new_item])
    }

    load_files(load_file, config, storage, progress).await
}
