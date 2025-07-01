//! This module contains tools that make use of raw terminal.
//!
//! Raw terminal can be enabled with [`enable_raw_mode`] and disabled with
//! [`disable_raw_mode`].
//!
//! Terminal in raw mode doesn't read input line by line, but all characters
//! are available as they are typed. On the other hand the interaction with the
//! user is no longer handled by the terminal, but needs to be handled manualy
//! in program (e.g. backspace, printing what is typed, ...). These codes can
//! be parsed with [`Terminal`] and the input can be handled with functions in
//! the module [`readers`].
//!
//! Raw mode is also useful when you need to request information from the
//! terminal with ansi codes. This can be handled with functions in the module
//! [`mod@request`].

mod io_provider;
mod stdio_provider;
mod sys;
mod terminal;
mod wait_for_in;

#[cfg(feature = "events")]
use std::{
    io::Write,
    time::{Duration, Instant},
};

#[cfg(feature = "events")]
use crate::{
    codes,
    raw::events::{AmbiguousEvent, AnyEvent, Event, Status},
};

pub use self::{
    io_provider::*, stdio_provider::*, sys::*, terminal::*, wait_for_in::*,
};

#[cfg(feature = "events")]
pub mod events;
#[cfg(feature = "readers")]
pub mod readers;
#[cfg(feature = "events")]
pub mod request;

#[cfg(feature = "events")]
pub(crate) fn raw_guard<T>(
    raw: bool,
    f: impl FnOnce() -> Result<T>,
) -> Result<T> {
    let is_raw = is_raw_mode_enabled();
    if raw != is_raw {
        if raw {
            enable_raw_mode()?;
        } else {
            disable_raw_mode()?;
        }
    }

    let res = f();

    if raw != is_raw {
        if raw {
            disable_raw_mode()?;
        } else {
            enable_raw_mode()?;
        }
    }

    res
}

/// Request response from the terminal. The response must match with the given
/// matching function `m`.
///
/// Many types of requests are directly implemented as functions in the module
/// [`mod@request`]. That should be suitable for most cases.
///
/// The argument to `m` is [`AmbiguousEvent`]. If you would like to skip
/// ambiguous and unknown events, use [`request()`].
///
/// This function will automatically enable raw mode for its duration.
///
/// The response is guarded with timeout and response to status request. So if
/// stdout or stdin is not terminal, the input will be consumed if available,
/// but the function will block for at most the given timeout. If the terminal
/// doesn't support the given code but supports status report, this function
/// will block until it receives the response (generally less than useful
/// timeout values).
///
/// # Errors
/// - [`crate::Error::Io`] io error when working with stdin and stdout.
/// - [`crate::Error::NotSupportedOnPlatform`] if raw mode is not supported on
///   this platform. It is supported on windows and unix (linux).
/// - [`crate::Error::StdInEof`] when stdin reaches eof.
///
/// ## Windows
/// - [`crate::Error::WaitAbandoned`] on windows when fails to wait for stdin.
///
/// # Example
/// ```no_run
/// use std::time::Duration;
///
/// use termal_core::{codes, Result, raw::{
///     request_ambiguous, Terminal,
///     events::{AmbiguousEvent, Event, Status, AnyEvent}
/// }};
///
/// let mut term = Terminal::stdio();
/// term.flushed(codes::CLEAR)?;
///
/// let evt = request_ambiguous(
///     codes::REQUEST_TERMINAL_NAME,
///     Duration::from_millis(100),
///     |e| {
///         if let AmbiguousEvent {
///             event: AnyEvent::Known(Event::Status(Status::TerminalName(n))),
///             ..
///         } = e
///         {
///             Some(n)
///         } else {
///             None
///         }
///     },
/// )?;
///
/// term.consume_available()?;
///
/// println!("{evt:#?}");
///
/// Result::Ok(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/raw/request_ambiguous.png)
#[cfg(feature = "events")]
pub fn request_ambiguous<T>(
    code: impl AsRef<str>,
    timeout: Duration,
    mut m: impl FnMut(AmbiguousEvent) -> Option<T>,
) -> Result<Option<T>> {
    raw_guard(true, || {
        let mut term = Terminal::stdio();
        term.write_all(code.as_ref().as_bytes())?;
        term.write_all(codes::REQUEST_STATUS_REPORT.as_bytes())?;
        term.flush()?;

        let mut now = Instant::now();
        let end_time = now + timeout;
        loop {
            let Some(evt) = term.read_ambiguous_timeout(end_time - now)?
            else {
                return Ok(None);
            };

            if matches!(
                evt,
                AmbiguousEvent {
                    event: AnyEvent::Known(Event::Status(Status::Ok)),
                    ..
                }
            ) {
                return Ok(None);
            }

            if let Some(res) = m(evt) {
                return Ok(Some(res));
            }

            now = Instant::now();
            if now >= end_time {
                return Ok(None);
            }
        }
    })
}

/// Request response from the terminal. The response must match with the given
/// matching function `m`.
///
/// Many types of requests are directly implemented as functions in the module
/// [`mod@request`]. That should be suitable for most cases.
///
/// The argument to `m` is [`Event`]. If you don't want to skip ambiguous and
/// unknown events, use [`request_ambiguous`].
///
/// This function will automatically enable raw mode for its duration.
///
/// The response is guarded with timeout and response to status request. So if
/// stdout or stdin is not terminal, the input will be consumed if available,
/// but the function will block for at most the given timeout. If the terminal
/// doesn't support the given code but supports status report, this function
/// will block until it receives the response (generally less than useful
/// timeout values).
///
/// # Errors
/// - [`crate::Error::Io`] io error when working with stdin and stdout.
/// - [`crate::Error::NotSupportedOnPlatform`] if raw mode is not supported on
///   this platform. It is supported on windows and unix (linux).
/// - [`crate::Error::StdInEof`] when stdin reaches eof.
///
/// ## Windows
/// - [`crate::Error::WaitAbandoned`] on windows when fails to wait for stdin.
///
/// # Example
/// ```no_run
/// use std::time::Duration;
///
/// use termal_core::{codes, Result, raw::{
///     request, Terminal,
///     events::{AmbiguousEvent, Event, Status, AnyEvent}
/// }};
///
/// let mut term = Terminal::stdio();
/// term.flushed(codes::CLEAR)?;
///
/// let evt = request(
///     codes::REQUEST_TERMINAL_NAME,
///     Duration::from_millis(100),
///     |e| {
///         if let Event::Status(Status::TerminalName(n)) = e {
///             Some(n)
///         } else {
///             None
///         }
///     },
/// )?;
///
/// term.consume_available()?;
///
/// println!("{evt:#?}");
///
/// Result::Ok(())
/// ```
///
/// ## Result in terminal
/// ![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/assets/raw/request_ambiguous.png)
#[cfg(feature = "events")]
pub fn request<T>(
    code: impl AsRef<str>,
    timeout: Duration,
    mut m: impl FnMut(Event) -> Option<T>,
) -> Result<Option<T>> {
    request_ambiguous(code, timeout, move |evt| match evt {
        AmbiguousEvent {
            event: AnyEvent::Known(evt),
            ..
        } => m(evt),
        _ => None,
    })
}
