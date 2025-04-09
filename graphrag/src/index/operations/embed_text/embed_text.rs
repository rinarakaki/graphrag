//! A module containing embed_text, load_strategy and create_row_from_embedding_data methods definition.

use std::collections::HashMap;
use std::iter::zip;

use log::info;

use ndarray;
use polars::prelude::LazyFrame;

use crate::cache::pipeline_cache::PipelineCache;
use crate::callbacks::workflow_callbacks::WorkflowCallbacks;
use crate::config::embeddings::create_collection_name;
use crate::index::operations::embed_text::strategies::typing::TextEmbeddingStrategy;
use crate::vector_stores::base::{BaseVectorStore, VectorStoreDocument};
use crate::vector_stores::factory::VectorStoreFactory;

// Per Azure OpenAI Limits
// https://learn.microsoft.com/en-us/azure/ai-services/openai/reference
const DEFAULT_EMBEDDING_BATCH_SIZE: usize = 500;

/// TextEmbedStrategyType class definition.
pub enum TextEmbedStrategyType {
    OpenAI,
    Mock,
}

impl TextEmbedStrategyType {
    pub fn as_str(&self) -> &str {
        match self {
            TextEmbedStrategyType::OpenAI => "openai",
            TextEmbedStrategyType::Mock => "mock",
        }
    }
}

impl std::fmt::Debug for TextEmbedStrategyType {
    /// Get a string representation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/**
Embed a piece of text into a vector space. The operation outputs a new column containing a mapping between doc_id and vector.

## Usage
```yaml
args:
    column: text # The name of the column containing the text to embed, this can either be a column with text, or a column with a list[tuple[doc_id, str]]
    to: embedding # The name of the column to output the embedding to
    strategy: <strategy config> # See strategies section below
```

## Strategies
The text embed operation uses a strategy to embed the text. The strategy is an object which defines the strategy to use. The following strategies are available:

### openai
This strategy uses openai to embed a piece of text. In particular it uses a LLM to embed a piece of text. The strategy config is as follows:

```yaml
strategy:
    type: openai
    llm: # The configuration for the LLM
        type: openai_embedding # the type of llm to use, available options are: openai_embedding, azure_openai_embedding
        api_key: !ENV ${GRAPHRAG_OPENAI_API_KEY} # The api key to use for openai
        model: !ENV ${GRAPHRAG_OPENAI_MODEL:gpt-4-turbo-preview} # The model to use for openai
        max_tokens: !ENV ${GRAPHRAG_MAX_TOKENS:6000} # The max tokens to use for openai
        organization: !ENV ${GRAPHRAG_OPENAI_ORGANIZATION} # The organization to use for openai
    vector_store: # The optional configuration for the vector store
        type: lancedb # The type of vector store to use, available options are: azure_ai_search, lancedb
        <...>
```
*/
pub async fn embed_text<T>(
    input: LazyFrame,
    callbacks: impl WorkflowCallbacks,
    cache: impl PipelineCache<T>,
    embed_column: &str,
    strategy: HashMap<String, String>,
    embedding_name: &str,
    // id_column: str = "id",
    // title_column: Option<str>,
) {
    let id_column = "id";
    let title_column = None;
    let vector_store_config = strategy.get("vector_store");

    if vector_store_config {
        let collection_name = _get_collection_name(vector_store_config, embedding_name);
        let vector_store = _create_vector_store(
            vector_store_config, collection_name
        );
        let vector_store_workflow_config = vector_store_config.get(
            embedding_name, vector_store_config
        );
        return _text_embed_with_vector_store(
            input,
            callbacks,
            cache,
            embed_column,
            strategy,
            vector_store,
            vector_store_config,
            id_column,
            title_column,
        ).await
    }

    _text_embed_in_memory(
        input,
        callbacks,
        cache,
        embed_column,
        strategy,
    ).await
}

async fn _text_embed_in_memory<T>(
    input: LazyFrame,
    callbacks: impl WorkflowCallbacks,
    cache: impl PipelineCache<T>,
    embed_column: &str,
    strategy: HashMap<String, String>,
) -> TextEmbeddingStrategy {
    let strategy_type = strategy["type"];
    let strategy_exec = load_strategy(strategy_type);
    let strategy_config = {**strategy};

    let texts: Vec<String> = input[embed_column].to_numpy().tolist();
    let result = strategy_exec(texts, callbacks, cache, strategy_config).await;

    result.embeddings
}

async fn _text_embed_with_vector_store<T>(
    input: LazyFrame,
    callbacks: impl WorkflowCallbacks,
    cache: impl PipelineCache<T>,
    embed_column: &str,
    strategy: HashMap<String, String>,
    vector_store: impl BaseVectorStore,
    vector_store_config: HashMap<String, String>,
    id_column: &str, // = "id",
    title_column: Option<str>,
) -> Vec<Vec<embedding>> {
    let strategy_type = strategy["type"];
    let strategy_exec = load_strategy(strategy_type);
    let strategy_config = {**strategy};

    // if max_retries is not set, inject a dynamically assigned value based on the total number of expected LLM calls to be made
    if strategy_config.get("llm") && strategy_config["llm"]["max_retries"] == -1 {
        strategy_config["llm"]["max_retries"] = input.len();
    }

    // Get vector-storage configuration
    let insert_batch_size: usize =
        vector_store_config.get("batch_size") || DEFAULT_EMBEDDING_BATCH_SIZE;

    let overwrite: bool = vector_store_config.get("overwrite", true);

    assert!(
        input.columns.contains(embed_column),
        format!("ValueError: Column {embed_column} not found in input LazyFrame with columns {input.columns}"),
    );
    let title = title_column.unwrap_or(embed_column);
    assert!(
        input.columns.contains(title),
        format!("ValueError: Column {title} not found in input LazyFrame with columns {input.columns}")
    );
    assert!(
        input.columns.contains(id_column),
        format!("ValueError: Column {id_column} not found in input LazyFrame with columns {input.columns}"),
    );

    let mut total_rows = 0;
    for row in input[embed_column] {
        if isinstance(row, list) {
            total_rows += len(row);
        } else {
            total_rows += 1;
        }
    }

    let mut i = 0;
    let mut starting_index = 0;

    let mut all_results = Vec::new();

    while insert_batch_size * i < input.shape[0] {
        let batch = input.iloc[insert_batch_size * i..insert_batch_size * (i + 1)];
        let texts: Vec<String> = batch[embed_column].to_numpy().tolist();
        let titles: Vec<String> = batch[title].to_numpy().tolist();
        let ids: Vec<String> = batch[id_column].to_numpy().tolist();
        let result = strategy_exec(texts, callbacks, cache, strategy_config).await;
        if result.embeddings {
            let embeddings = result.embeddings.iter().filter(|embedding| embedding.is_some()).collect();
            all_results.extend(embeddings);
        }

        let vectors = result.embeddings.unwrap_or_default();
        let documents = Vec::<VectorStoreDocument>::new();
        for (doc_id, doc_text, doc_title, doc_vector) in zip(
            ids, texts, titles, vectors, strict=true
        ) {
            if type(doc_vector) is np.ndarray {
                doc_vector = doc_vector.tolist()
            }
            let document = VectorStoreDocument {
                id: doc_id,
                text: doc_text,
                vector: doc_vector,
                attributes: {"title": doc_title},
            };
            documents.push(document);
        }

        vector_store.load_documents(documents, overwrite && i == 0);
        starting_index += documents.len();
        i += 1;
    }

    all_results
}

fn _create_vector_store(
    vector_store_config: dict, collection_name: str
) -> impl BaseVectorStore {
    let vector_store_type: str = str(vector_store_config.get("type"));
    if collection_name {
        vector_store_config.update({"collection_name": collection_name});
    }

    let vector_store = VectorStoreFactory().create_vector_store(
        vector_store_type, kwargs=vector_store_config
    );

    vector_store.connect(**vector_store_config);
    vector_store
}

fn _get_collection_name(vector_store_config: dict, embedding_name: &str) -> str {
    let container_name = vector_store_config.get("container_name", "default");
    let collection_name = create_collection_name(container_name, embedding_name);

    info!("using vector store {vector_store_config.get('type')} with container_name {container_name} for embedding {embedding_name}: {collection_name}");
    collection_name
}

/// Load strategy method definition.
fn load_strategy(strategy: TextEmbedStrategyType) -> TextEmbeddingStrategy {
    match strategy {
        TextEmbedStrategyType::OpenAI => {
            use crate::index::operations::embed_text::strategies::openai::run as run_openai;

            run_openai
        }
        TextEmbedStrategyType::Mock => {
            use crate::index::operations::embed_text::strategies::mock::run as run_mock;

            run_mock
        }
    }
}
