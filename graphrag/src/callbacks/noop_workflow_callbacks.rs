//! A no-op implementation of WorkflowCallbacks.

use crate::callbacks::workflow_callbacks::WorkflowCallbacks;

/// A no-op implementation of WorkflowCallbacks.
pub struct NoopWorkflowCallbacks;

impl WorkflowCallbacks for NoopWorkflowCallbacks {}
