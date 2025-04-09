//! API for GraphRAG.
//!
//! WARNING: This API is under development and may undergo changes in future releases.
//! Backwards compatibility is not guaranteed at this time.

pub mod index;
pub mod prompt_tune;
pub mod query;

pub use crate::api::index::build_index;
pub use crate::api::prompt_tune::generate_indexing_prompts;
pub use crate::api::query::{
    basic_search,
    basic_search_streaming,
    drift_search,
    drift_search_streaming,
    global_search,
    global_search_streaming,
    local_search,
    local_search_streaming,
    multi_index_basic_search,
    multi_index_drift_search,
    multi_index_global_search,
    multi_index_local_search,
};
pub use crate::prompt_tune::types::DocSelectionType;
