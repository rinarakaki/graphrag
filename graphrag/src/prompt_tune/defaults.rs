//! Default values for the prompt-tuning module.
//!
//! Note: These values get accessed from the CLI to set default behavior.
//! To maintain fast responsiveness from the CLI, do not add long-running code in this file and be mindful of imports.

pub const DEFAULT_TASK: &str = "
Identify the relations and structure of the community of interest, specifically within the {domain} domain.
";

pub const K: usize = 15;
pub const LIMIT: usize = 15;
pub const MAX_TOKEN_COUNT: usize = 2000;
pub const MIN_CHUNK_SIZE: usize = 200;
pub const N_SUBSET_MAX: usize = 300;
pub const MIN_CHUNK_OVERLAP: usize = 0;
pub const PROMPT_TUNING_MODEL_ID: &str = "default_chat_model";
