use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("End of file was reached on stdin")]
    StdInEof,
    #[error("{0} is not supported on this platform.")]
    NotSupportedOnPlatform(&'static str),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
