//! Pipeline state types.

use std::any::Any;
use std::collections::HashMap;

pub type PipelineState = HashMap<dyn Any, dyn Any>;
