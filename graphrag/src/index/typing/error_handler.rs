//! Shared error handler types.

use std::collections::HashMap;

pub type ErrorHandlerFn = fn(Option<dyn std::error::Error>, Option<&str>, Option<HashMap<String, String>>);
