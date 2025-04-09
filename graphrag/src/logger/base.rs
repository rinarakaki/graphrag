//! Base classes for logging and progress reporting.

use std::any::Any;
use std::collections::HashMap;

use crate::logger::progress::Progress;

/// Provides a way to log status updates from the pipeline.
pub trait StatusLogger {
    /// Log an error.
    fn error(
        &self,
        message: &str,
        details: Option<HashMap<String, Box<dyn Any>>>, /* = None */
    );

    /// Log a warning.
    fn warning(
        &self,
        message: &str,
        details: Option<HashMap<String, Box<dyn Any>>>, /* = None */
    );

    /// Report a log.
    fn log(&self, message: &str, details: Option<HashMap<String, Box<dyn Any>>> /* = None */);
}

/// Abstract base class for progress loggers.
///
/// This is used to report workflow processing progress via mechanisms like progress-bars.
pub trait ProgressLogger {
    /// Update progress.
    fn __call__(&self, update: Progress);

    /// Dispose of the progress logger.
    fn dispose(&self);

    /// Create a child progress bar.
    fn child(&self, prefix: &str, transient: bool /* = True */) -> Box<dyn ProgressLogger>;

    /// Force a refresh.
    fn force_refresh(&self);

    /// Stop the progress logger.
    fn stop(&self);

    /// Log an error.
    fn error(&self, message: &str);

    /// Log a warning.
    fn warning(&self, message: &str);

    /// Log information.
    fn info(&self, message: &str);

    /// Log success.
    fn success(&self, message: &str);
}
