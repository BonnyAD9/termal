use std::time::Duration;

/// Decides how often progress should update. This can be based either on
/// iterations or time.
///
/// The default is every 100 ms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpdatePolicy {
    Iterations(usize),
    Time(Duration),
}

impl Default for UpdatePolicy {
    fn default() -> Self {
        Self::Time(Duration::from_millis(100))
    }
}

impl From<usize> for UpdatePolicy {
    fn from(value: usize) -> Self {
        Self::Iterations(value)
    }
}

impl From<Duration> for UpdatePolicy {
    fn from(value: Duration) -> Self {
        Self::Time(value)
    }
}
