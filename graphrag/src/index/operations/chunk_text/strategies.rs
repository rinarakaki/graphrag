//! A module containing chunk strategies.

//::nltk
use tiktoken_rs;

use crate::config::models::chunking_config::ChunkingConfig;
use crate::index::operations::chunk_text::typing::TextChunk;
use crate::index::text_splitting::text_splitting::{Tokenizer, split_multiple_texts_on_tokens};
use crate::logger::progress::ProgressTicker;

/// Get the encoding model.
pub fn get_encoding_fn(encoding_name: &str) -> (fn(&str) -> Vec<u32>, fn(&[u32]) -> Vec<u8>) {
    let enc = tiktoken_rs::get_encoding(encoding_name);

    fn encode(text: &str) -> Vec<u32> {
        return enc::encode(text)
    }

    fn decode(tokens: &[u32]) -> Vec<u8> {
        enc::decode(tokens)
    }

    (encode, decode)
}

/// Chunks text into chunks based on encoding tokens.
pub fn run_tokens(
    input: Vec<String>,
    config: ChunkingConfig,
    tick: ProgressTicker,
) -> Iterable<TextChunk> {
    let tokens_per_chunk = config.size;
    let chunk_overlap = config.overlap;
    let encoding_name = &config.encoding_model;

    let (encode, decode) = get_encoding_fn(encoding_name);
    split_multiple_texts_on_tokens(
        input,
        Tokenizer {
            chunk_overlap,
            tokens_per_chunk,
            encode,
            decode,
        },
        tick,
    )
}

/// Chunks text into multiple parts by sentence.
pub fn run_sentences(
    input: Vec<String>, _config: ChunkingConfig, tick: ProgressTicker
) -> Fn() {
    #[coroutine]
    || {
        for (doc_idx, text) in input.iter().enumerate() {
            let sentences = nltk.sent_tokenize(text);
            for sentence in sentences {
                yield TextChunk {
                    text_chunk: sentence,
                    source_doc_indices: vec![doc_idx],
                    n_tokens: None,
                }
            }
            tick(1);
        }
    }
}
