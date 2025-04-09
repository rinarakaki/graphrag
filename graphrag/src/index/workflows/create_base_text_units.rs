//! A module containing run_workflow method definition.

use polars::prelude::{LazyFrame, Series, SortMultipleOptions};

use crate::callbacks::workflow_callbacks::WorkflowCallbacks;
use crate::config::models::chunking_config::ChunkStrategyType;
use crate::config::models::graph_rag_config::GraphRagConfig;
use crate::index::operations::chunk_text::chunk_text::chunk_text;
use crate::index::operations::chunk_text::strategies::get_encoding_fn;
use crate::index::typing::context::PipelineRunContext;
use crate::index::typing::workflow::WorkflowFunctionOutput;
use crate::index::utils::hashing::gen_sha512_hash;
use crate::logger::progress::Progress;
use crate::utils::storage::{load_table_from_storage, write_table_to_storage};

/// All the steps to transform base text_units.
pub async fn run_workflow(
    config: GraphRagConfig,
    context: PipelineRunContext,
) -> WorkflowFunctionOutput {
    let documents = load_table_from_storage("documents", context.storage).await;

    let chunks = config.chunks;

    let output = create_base_text_units(
        documents,
        context.callbacks,
        chunks.group_by_columns,
        chunks.size,
        chunks.overlap,
        chunks.encoding_model,
        chunks.strategy,
        chunks.prepend_metadata,
        chunks.chunk_size_includes_metadata,
    );

    write_table_to_storage(output, "text_units", context.storage).await;

    WorkflowFunctionOutput {
        result: output,
    }
}

/// All the steps to transform base text_units.
pub fn create_base_text_units(
    documents: LazyFrame,
    callbacks: WorkflowCallbacks,
    group_by_columns: Vec<String>,
    size: usize,
    overlap: usize,
    encoding_model: &str,
    strategy: ChunkStrategyType,
    prepend_metadata: bool, // = false,
    chunk_size_includes_metadata: bool, // = false,
) -> LazyFrame {
    let sort = documents.sort(["id"], SortMultipleOptions {
        descending: vec![false],
        ..Default::default()
    });

    let sort["text_with_ids"] = list(
        zip(*[sort[col] for col in ["id", "text"]], strict=true)
    );

    callbacks.progress(Progress {
        percent: Some(0.0),
        ..Default::default()
    });

    let agg_dict = {"text_with_ids": list};
    if "metadata" in documents {
        agg_dict["metadata"] = "first";
    }

    let aggregated = (
        (
            sort.group_by(group_by_columns, sort=false)
            if group_by_columns.len() > 0
            else sort.group_by(lambda _x: true)
        )
        .agg(agg_dict)
        .reset_index()
    );
    aggregated.rename(columns={"text_with_ids": "texts"}, inplace=true);

    fn chunker(row: HashMap<String, Box<dyn Any>>) -> Any {
        line_delimiter = ".\n"
        metadata_str = ""
        metadata_tokens = 0

        if prepend_metadata and "metadata" in row:
            metadata = row["metadata"]
            if isinstance(metadata, str):
                metadata = json.loads(metadata)
            if isinstance(metadata, dict):
                metadata_str = (
                    line_delimiter.join(format!("{k}: {v}") for k, v in metadata.items())
                    + line_delimiter
                )

            if chunk_size_includes_metadata:
                encode, _ = get_encoding_fn(encoding_model)
                metadata_tokens = len(encode(metadata_str))
                if metadata_tokens >= size:
                    message = "Metadata tokens exceeds the maximum tokens per chunk. Please increase the tokens per chunk."
                    raise ValueError(message)

        chunked = chunk_text(
            LazyFrame([row]).reset_index(drop=true),
            column="texts",
            size=size - metadata_tokens,
            overlap=overlap,
            encoding_model=encoding_model,
            strategy=strategy,
            callbacks=callbacks,
        )[0]

        if prepend_metadata:
            for index, chunk in enumerate(chunked):
                if isinstance(chunk, str):
                    chunked[index] = metadata_str + chunk
                else:
                    chunked[index] = (
                        (chunk[0], metadata_str + chunk[1], chunk[2]) if chunk else None
                    )

        row["chunks"] = chunked
        return row
    }

    let aggregated = aggregated.apply(lambda row: chunker(row), axis=1);

    let aggregated = cast("LazyFrame", aggregated[[*group_by_columns, "chunks"]])
    let aggregated = aggregated.explode("chunks");
    aggregated.rename(
        columns={
            "chunks": "chunk",
        },
        inplace=true,
    );
    aggregated["id"] = aggregated.apply(
        lambda row: gen_sha512_hash(row, ["chunk"]), axis=1
    );
    aggregated[["document_ids", "chunk", "n_tokens"]] = LazyFrame(
        aggregated["chunk"].tolist(), index=aggregated.index
    );
    // rename for downstream consumption
    aggregated.rename(columns={"chunk": "text"}, inplace=true);

    cast(
        "LazyFrame", aggregated[aggregated["text"].notna()].reset_index(drop=true)
    )
}
