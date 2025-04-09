//! Progress reporting types.

/// A class representing the progress of a task.
#[derive(Default)]
pub struct Progress {
    /// 0 - 1 progress
    pub percent: Option<f64>,

    /// Description of the progress
    pub description: Option<String>,

    /// Total number of items
    pub total_items: Option<usize>,

    /// Number of items completed ""
    pub completed_items: Option<usize>,
}

/// A function to handle progress reports.
pub type ProgressHandler = fn(Progress);

/// A class that emits progress reports incrementally.
pub struct ProgressTicker {
    _callback: Option<ProgressHandler>,
    _num_total: usize,
    _num_complete: usize,
}

impl ProgressTicker {
    pub fn new(callback: Option<ProgressHandler>, num_total: usize) -> Self {
        ProgressTicker {
            _callback: callback,
            _num_total: num_total,
            _num_complete: 0,
        }
    }

    // def __call__(self, num_ticks: int = 1) -> None:
    //     /// Emit progress.
    //     self._num_complete += num_ticks
    //     if self._callback is not None:
    //         self._callback(
    //             Progress(
    //                 total_items=self._num_total, completed_items=self._num_complete
    //             )
    //         )

    // pub fn done(&self) {
    //     /// Mark the progress as done.
    //     if self._callback is not None:
    //         self._callback(
    //             Progress(total_items=self._num_total, completed_items=self._num_total)
    //         )
    // }
}

/// Create a progress ticker.
pub fn progress_ticker(callback: Option<ProgressHandler>, num_total: usize) -> ProgressTicker {
    ProgressTicker::new(callback, num_total)
}

// pub fn progress_iterable<T>(
//     iterable: Iterable[T],
//     progress: ProgressHandler | None,
//     num_total: int | None = None,
// ) -> Iterable[T] {
//     /// Wrap an iterable with a progress handler. Every time an item is yielded, the progress handler will be called with the current progress.
//     if num_total.is_none():
//         num_total = len(list(iterable))

//     tick = ProgressTicker(progress, num_total)

//     for item in iterable:
//         tick(1)
//         yield item
// }
