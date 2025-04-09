//! A module containing the 'Tokenizer', 'TextSplitter', 'NoopTextSplitter' and 'TokenTextSplitter' models.

use std::collections::HashSet;

use log::error;
use polars::prelude;
use tiktoken_rs;

use crate::config::defaults as defs;
use crate::index::operations::chunk_text::typing::TextChunk;
use crate::logger::progress::ProgressTicker;

pub type EncodedText = Vec<u32>;
pub type DecodeFn = fn(EncodedText) -> String;
pub type EncodeFn<'a> = fn(&'a str) -> EncodedText;
pub type LengthFn = fn(String) -> usize;

// TODO(rinarakaki) @dataclass(frozen=True)
/// Tokenizer data class.
pub struct Tokenizer {
    /// Overlap in tokens between chunks
    pub chunk_overlap: usize,
    /// Maximum number of tokens per chunk
    pub tokens_per_chunk: usize,
    ///  Function to decode a list of token ids to a string
    pub decode: DecodeFn,
    ///  Function to encode a string to a list of token ids
    pub encode: EncodeFn,
}

/// Text splitter class definition.
pub struct BaseTextSplitter {
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub length_function: LengthFn,
    pub keep_separator: bool,
    pub add_start_index: bool,
    pub strip_whitespace: bool,
}

impl BaseTextSplitter {
    /// Init method definition.
    fn new(
        // based on text-ada-002-embedding max input buffer length
        // https://platform.openai.com/docs/guides/embeddings/second-generation-models
        chunk_size: usize,
        chunk_overlap: usize,
        length_function: LengthFn,
        keep_separator: bool,
        add_start_index: bool,
        strip_whitespace: bool,
    ) -> Self {
        BaseTextSplitter {
            chunk_size,
            chunk_overlap,
            length_function,
            keep_separator,
            add_start_index,
            strip_whitespace,
        }
    }
}

impl Default for BaseTextSplitter {
    fn default() -> Self {
        BaseTextSplitter {
            chunk_size: 8191,
            chunk_overlap: 100,
            length_function: |text| text.len(),
            keep_separator: false,
            add_start_index: false,
            strip_whitespace: true,
        }
    }
}

pub trait TextSplitter {
    /// Split text method definition.
    fn split_text(&self, text: Vec<String>) -> impl IntoIterator<Item=String>;
}

/// Noop text splitter class definition.
pub struct NoopTextSplitter {
    pub base: BaseTextSplitter,
}

impl TextSplitter for NoopTextSplitter {
    /// Split text method definition.
    fn split_text(self, text: Vec<String>) -> impl Iterator<Item = String> {
        text
    }
}

/// Token text splitter class definition.
pub struct TokenTextSplitter {
    pub base: BaseTextSplitter,
    _tokenizer: tiktoken_rs::CoreBPE,
}

impl TokenTextSplitter {
    /// Init method definition.
    pub fn new(
        base: BaseTextSplitter,
        encoding_name: String, // = defs.ENCODING_MODEL,
        model_name: Option<String>, // = None,
    ) -> TokenTextSplitter {
        TokenTextSplitter {
            base,
            _tokenizer: match model_name {
                Some(model_name) => match tiktoken_rs::get_bpe_from_model(model_name) {
                    Ok(enc) => enc,
                    Err(_) => {
                        error!("Model %s not found, using %s", model_name, encoding_name);
                        tiktoken_rs::get_encoding(encoding_name)
                    }
                },
                None => tiktoken_rs::get_encoding(encoding_name),
            },
        }
    }

    /// Encode the given text into an int-vector.
    pub fn encode(self, text: &str) -> Vec<tiktoken_rs::Rank> {
        self._tokenizer.encode(
            text,
            HashSet::new(),
        )
    }

    /// Return the number of tokens in a string.
    pub fn num_tokens(self, text: &str) -> usize {
        self.encode(text).len()
    }
}

impl TextSplitter for TokenTextSplitter {
    /// Split text method.
    fn split_text(self, text: Vec<String>) -> Vec<String> {
        let text = text.join(" ");

        let tokenizer = Tokenizer {
            chunk_overlap: self.chunk_overlap,
            tokens_per_chunk: self.chunk_size,
            decode: self._tokenizer.decode,
            encode: |text| self.encode(text),
        };

        split_single_text_on_tokens(text, tokenizer)
    }
}

/// Split a single text and return chunks using the tokenizer.
pub fn split_single_text_on_tokens(text: &str, tokenizer: Tokenizer) -> Vec<String> {
    let mut result = Vec::new();
    let input_ids = tokenizer.encode(text);

    let mut start_idx = 0;
    let mut cur_idx = min(start_idx + tokenizer.tokens_per_chunk, input_ids.len());
    let mut chunk_ids = input_ids[start_idx..cur_idx];

    while start_idx < input_ids.len() {
        let chunk_text = tokenizer.decode(list(chunk_ids));
        result.push(chunk_text);  // push chunked text as string
        start_idx += tokenizer.tokens_per_chunk - tokenizer.chunk_overlap;
        cur_idx = min(start_idx + tokenizer.tokens_per_chunk, input_ids.len());
        chunk_ids = input_ids[start_idx..cur_idx];
    }

    result
}

// Adapted from - https://github.com/langchain-ai/langchain/blob/77b359edf5df0d37ef0d539f678cf64f5557cb54/libs/langchain/langchain/text_splitter.py#L471
// So we could have better control over the chunking process
/// Split multiple texts and return chunks with metadata using the tokenizer.
pub fn split_multiple_texts_on_tokens(
    texts: Vec<String>, tokenizer: Tokenizer, tick: ProgressTicker
) -> Vec<TextChunk> {
    let mut result = Vec::new();
    let mut mapped_ids = Vec::new();

    for (source_doc_idx, text) in texts.iter().enumerate() {
        let encoded = tokenizer.encode(text);
        if tick {
            tick(1);  // Track progress if tick callback is provided
        }
        mapped_ids.push((source_doc_idx, encoded));
    }

    let input_ids = mapped_ids
        .iter()
        .flat_map(|(source_doc_idx, ids)| ids)
        .collect::<Vec<_>>();

    let mut start_idx = 0;
    let mut cur_idx = min(start_idx + tokenizer.tokens_per_chunk, input_ids.len());
    let mut chunk_ids = input_ids[start_idx..cur_idx];

    while start_idx < input_ids.len() {
        let chunk_text = tokenizer.decode(chunk_ids.iter().map(|(_, id)| id).collect());
        let doc_indices = chunk_ids.iter().map(|(doc_idx, _)| doc_idx).collect();
        result.push(TextChunk {
            chunk_text,
            source_doc_indices: doc_indices,
            n_tokens: chunk_ids.len(),
        });
        start_idx += tokenizer.tokens_per_chunk - tokenizer.chunk_overlap;
        cur_idx = min(start_idx + tokenizer.tokens_per_chunk, len(input_ids));
        chunk_ids = input_ids[start_idx..cur_idx];
    }

    result
}
