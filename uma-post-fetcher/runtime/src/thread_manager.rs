/// A simple thread manager that schedules tasks in FIFO order.  In this
/// example there is only one logical task per invocation, so the
/// implementation is trivial.  In a more complex runtime this would
/// coordinate multiple tasks and manage cooperative scheduling.
pub struct ThreadManager;

impl ThreadManager {
    pub fn new() -> Self {
        Self
    }

    /// Run a closure synchronously on the current thread.  Returns the
    /// closure's result.  Errors can be propagated via the closure's return
    /// type.
    pub fn run_sync<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        f()
    }
}