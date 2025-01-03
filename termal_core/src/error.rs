use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Error type of termal.
#[derive(Debug, Error)]
pub enum Error {
    /// Eof was reached on stdin.
    #[error("End of file was reached on stdin")]
    StdInEof,
    /// The given action is not supported on this platform.
    #[error("{0} is not supported on this platform.")]
    NotSupportedOnPlatform(&'static str),
    /// Failed to wait for stdin (on windows).
    #[error("Failed to wait for stdin.")]
    WaitAbandoned,
    /// Any IO error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
