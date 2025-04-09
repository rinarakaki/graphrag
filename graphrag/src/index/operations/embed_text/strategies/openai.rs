//! A module containing run method definition.

use std::any::Any;
use std::collections::HashMap;

use log::info;
use ndarray;

use crate::cache::pipeline_cache::PipelineCache;
use crate::callbacks::workflow_callbacks::WorkflowCallbacks;
use crate::config::models::language_model_config::LanguageModelConfig;
use crate::index::operations::embed_text::strategies::typing::TextEmbeddingResult;
use crate::index::text_splitting::text_splitting::TokenTextSplitter;
use crate::index::utils::is_null::is_null;
use crate::language_model::manager::ModelManager;
use crate::language_model::protocol::base::EmbeddingModel;
use crate::logger::progress::{ProgressTicker, progress_ticker};

/// Run the Claim extraction chain.
async fn run<T>(
    input: Vec<String>,
    callbacks: impl WorkflowCallbacks,
    cache: impl PipelineCache<T>,
    args: HashMap<String, Box<dyn Any>>,
) -> TextEmbeddingResult {
    if is_null(input) {
        return TextEmbeddingResult {
            embeddings: None,
        };
    }

    let batch_size = args.get("batch_size", 16);
    let batch_max_tokens = args.get("batch_max_tokens", 8191);
    let llm_config = args["llm"];
    let llm_config = LanguageModelConfig(**args["llm"]);
    let splitter = _get_splitter(llm_config, batch_max_tokens);
    let model = ModelManager::new().get_or_create_embedding_model(
        name="text_embedding",
        model_type=llm_config.r#type,
        config=llm_config,
        callbacks=callbacks,
        cache=cache,
    );
    let semaphore = asyncio.Semaphore(args.get("num_threads", 4));

    // Break up the input texts. The sizes here indicate how many snippets are in each input text
    let (texts, input_sizes) = _prepare_embed_texts(input, splitter);
    let text_batches = _create_text_batches(
        texts,
        batch_size,
        batch_max_tokens,
        splitter,
    );
    info!(
        "embedding %d inputs via %d snippets using %d batches. max_batch_size=%d, max_tokens=%d",
        input.len(),
        texts.len(),
        text_batches.len(),
        batch_size,
        batch_max_tokens,
    );
    let ticker = progress_ticker(callbacks.progress, len(text_batches));

    // Embed each chunk of snippets
    let embeddings = _execute(model, text_batches, ticker, semaphore).await;
    let embeddings = _reconstitute_embeddings(embeddings, input_sizes);

    TextEmbeddingResult { embeddings }
}

fn _get_splitter(
    config: LanguageModelConfig, batch_max_tokens: int
) -> TokenTextSplitter {
    TokenTextSplitter::new(
        encoding_name=config.encoding_model,
        chunk_size=batch_max_tokens,
    )
}

async fn _execute(
    model: EmbeddingModel,
    chunks: Vec<Vec<String>>,
    tick: ProgressTicker,
    semaphore: asyncio.Semaphore,
) -> Vec<Vec<f64>> {
    async fn embed(chunk: Vec<String>) -> Array {
        async with semaphore {
            let chunk_embeddings = model.aembed_batch(chunk).await;
            let result = np.array(chunk_embeddings);
            tick(1);
        }
        result
    }

    let futures = chunks.iter().map(|chunk| embed(chunk)).collect();
    let results = asyncio.gather(*futures).await;
    // merge results in a single list of lists (reduce the collect dimension)
    [item for sublist in results for item in sublist]
}

/// Create batches of texts to embed.
fn _create_text_batches(
    texts: Vec<String>,
    max_batch_size: int,
    max_batch_tokens: int,
    splitter: TokenTextSplitter,
) -> Vec<Vec<String>> {
    // https://learn.microsoft.com/en-us/azure/ai-services/openai/reference
    // According to this embeddings reference, Azure limits us to 16 concurrent embeddings and 8191 tokens per request
    let mut result = Vec::new();
    let mut current_batch = Vec::new();
    let mut current_batch_tokens = 0;

    for text in texts.iter() {
        let token_count = splitter.num_tokens(text);
        if (
            current_batch.len() >= max_batch_size
            || current_batch_tokens + token_count > max_batch_tokens
        ) {
            result.push(current_batch);
            current_batch = Vec::new();
            current_batch_tokens = 0;
        }

        current_batch.push(text);
        current_batch_tokens += token_count;
    }

    if !current_batch.is_empty() {
        result.push(current_batch);
    }

    result
}

fn _prepare_embed_texts(
    input: Vec<String>, splitter: TokenTextSplitter,
) -> (Vec<String>, Vec<usize>) {
    let mut sizes = Vec::<usize>::new();
    let mut snippets = Vec::<String>::new();

    for text in input.iter() {
        // Split the input text and filter out any empty content
        let split_texts = splitter.split_text(text);
        if let None = split_texts {
            continue;
        }
        let split_texts = split_texts.iter().filter(|text| !text.is_empty()).collect();

        sizes.push(split_texts.len());
        snippets.extend(split_texts);
    }

    (snippets, sizes)
}

/// Reconstitute the embeddings into the original input texts.
fn _reconstitute_embeddings(
    raw_embeddings: Vec<Vec<f64>>, sizes: Vec<int>,
) -> Vec<Option<Vec<f64>>> {
    let mut embeddings = Vec::<Option<Vec<f64>>>::new();
    let cursor = 0;
    for size in sizes {
        if size == 0 {
            embeddings.push(None);
        } else if size == 1 {
            let embedding = raw_embeddings[cursor];
            embeddings.push(Some(embedding));
            cursor += 1;
        } else {
            chunk = raw_embeddings[cursor..cursor + size];
            average = np.average(chunk, axis=0);
            normalized = average / np.linalg.norm(average);
            embeddings.push(normalized.tolist());
            cursor += size;
        }
    }
    embeddings
}
