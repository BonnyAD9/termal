/// Represents state that can be converted to progress.
pub trait StateProgress {
    fn progress(&self) -> f32;
}
