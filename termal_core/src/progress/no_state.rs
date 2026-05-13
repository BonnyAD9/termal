use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct NoState;

impl Display for NoState {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
