//! Storage functions for the GraphRAG run module.

use log::{error, info};
// from io::BytesIO

use polars::prelude::LazyFrame;

use crate::storage::pipeline_storage::PipelineStorage;

/// Load a parquet from the storage instance.
pub async fn load_table_from_storage(name: &str, storage: PipelineStorage) -> LazyFrame {
    let filename = format!("{name}.parquet");
    if !storage.has(filename).await {
        let msg = format!("Could not find {filename} in storage!");
        raise ValueError(msg)
    }
    info!("reading table from storage: {filename}");
    match pd.read_parquet(BytesIO(storage.get(filename, as_bytes=true).await)) {
        Ok(df) => df,
        Err(e) => {
            error!("error loading table from storage: {filename}");
            panic!();
        }
    }
}

/// Write a table to storage.
pub async fn write_table_to_storage(
    table: LazyFrame, name: &str, storage: PipelineStorage
) {
    storage.set(format!("{name}.parquet"), table.to_parquet(), None).await
}

/// Delete a table to storage.
pub async fn delete_table_from_storage(name: &str, storage: PipelineStorage) {
    storage.delete(format!("{name}.parquet")).await
}

/// Check if a table exists in storage.
pub async fn storage_has_table(name: &str, storage: PipelineStorage) -> bool {
    storage.has(format!("{name}.parquet")).await
}
