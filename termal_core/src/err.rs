//! This module contains error types for the whole crate termal.
//!
//! There is only one error type [`Error`] which is enumeration of all the
//! possible error codes.
//!
//! There is also alias for result with this error [`Result`] which is used
//! throught the whole crate termal.
//!
//! Only exception are proc macros implementations which have their own error
//! type for compile time errors.

use thiserror::Error;

/// Result type for termal.
///
/// It is [`std::result::Result`] where error is
/// [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Error type of termal. It is the only runtime error type used in termal.
#[derive(Debug, Error)]
pub enum Error {
    /// Eof was reached on stdin.
    ///
    /// This special error is returned when you try
    /// to read from terminal and it signals eof. This is not necessarily error
    /// situation, but it is much esier to handle this case with error type
    /// than always having to return `Result<Option>`. On the other hand, this
    /// is in many cases unexpected and may be considered error.
    #[error("End of file was reached on stdin")]
    StdInEof,
    /// The given action is not supported on this platform. This may be
    /// returned by any function in the module [`crate::raw::sys`]. The
    /// functions are mostly implemented only for unix and windows.
    #[error("{0} is not supported on this platform.")]
    NotSupportedOnPlatform(&'static str),
    /// Failed to wait for stdin (on windows).
    ///
    /// This may happen on windows if
    /// the thread that locked stdin exited without releasing the stdin handle.
    ///
    /// In correctly working program, this should never happen. If it happens
    /// something is very wrong and you may want to exit the program (maybe
    /// panic). Termal doesn't panic in this case to give you the option to not
    /// panic.
    #[error("Failed to wait for stdin.")]
    WaitAbandoned,
    /// Failed to parse rgb string.
    ///
    /// This error is returned by the function
    /// [`crate::FromColorStr::from_color_str`]. If the parse fails when
    /// parsing event, it is considered as unkown event.
    #[error("Failed to parse rgb.")]
    InvalidRgbFormat,
    /// Any IO error.
    ///
    /// For example when reading from terminal.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Failed to parse int.
    ///
    /// Not returned by any termal function. It is used internaly when parsing
    /// messages coming from terminal.
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}
