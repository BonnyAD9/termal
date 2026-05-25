use std::fmt::Display;

/// No state when tracking progress. This is the default.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoState;

impl Display for NoState {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
