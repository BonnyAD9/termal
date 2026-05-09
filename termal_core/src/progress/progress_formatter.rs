use std::time::Duration;

/// Decides how to display progress.
pub trait ProgressFormatter {
    /// Start the progress. This may not be called.
    fn start(&mut self, task: &str, info: &str);

    /// Update the progress.
    fn update(&mut self, done: f32, task: &str, info: &str, eta: Duration);

    /// Finish the progress.
    fn finish(&mut self, task: &str, info: &str, time: Duration);
}
