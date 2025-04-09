//! Token limit method definition.

use crate::config::defaults as defs;
use crate::index::text_splitting::text_splitting::{BaseTextSplitter, TokenTextSplitter};

/// Check token limit.
pub fn check_token_limit(text: Vec<String>, max_token: usize) -> usize {
    let text_splitter = TokenTextSplitter::new(
        BaseTextSplitter {
            chunk_size: max_token,
            chunk_overlap: 0,
            ..Default::default()
        },
        defs::ENCODING_MODEL,
        None,
    );
    let docs = text_splitter.split_text(text);
    if !docs.is_empty() {
        return 0;
    }
    return 1;
}
