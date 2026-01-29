use std::time::{Duration, Instant};

use minlin::Vec2;

use crate::{
    Error, Result, Rgb,
    codes::{self, Selection},
    raw::{
        Terminal,
        events::{Event, Status, TermAttr},
        raw_guard, request,
    },
};

macro_rules! impl_request {
    ($(
        $(? $($doc:literal)*)?
        $fname:ident = $cname:ident
            $(!($($argm:ident : $atm:ty),* $(,)?))?
            $( ($($argf:ident : $atf:ty),* $(,)?))?
        -> $tname:ty { $mat:pat => $n:expr $(,)? } $(,)?
    )*) => {
        $(
            $($(#[doc = $doc])*)?
            pub fn $fname(
                $($($argm: $atm, )*)?
                $($($argf: $atf, )*)?
                timeout: Duration
            ) -> Result<$tname> {
                request(
                    codes::$cname
                        $(!($($argm),*))?
                        $( ($($argf),*))?,
                    timeout,
                    |e| {
                        match e {
                            $mat => Some($n),
                            _ => None,
                        }
                    }
                ).transpose().ok_or(Error::NoResponse)?
            }
        )*
    };
}

impl_request!(
    ? "Requests the device attributes. Result is the type [`TermAttr`] with"
    "terminal type and and supported features."
    ""
    "Uses the code [`codes::REQUEST_DEVICE_ATTRIBUTES`]."
    ""
    "This will guard the response with status report and timeout. It will wait"
    "for the expected response. This will return error if response to status"
    "report will be received before the expected response or if the timeout"
    "passes."
    ""
    "Note that the status report is not consumed on success."
    ""
    "# Errors"
    "- [`Error::NoResponse`] if status report is received or if the timeout"
    "  passes."
    "- [`Error::Io`] on io error when working with stdin and stdout."
    "- [`Error::NotSupportedOnPlatform`] if raw mode is not supported on this"
    "  platform. It is supported on windows and unix (linux)."
    "- [`Error::StdInEof`] when stdin reaches eof."
    ""
    "## Windows"
    "- [`Error::WaitAbandoned`] when fails to wait for stdin. See the error"
    "  documentation for more info."
    ""
    "# Example"
    "```no_run"
    "use std::time::Duration;"
    "use termal_core::{codes, raw::{Terminal, request}, Result};"
    ""
    "let mut term = Terminal::stdio();"
    "term.flushed(codes::CLEAR)?;"
    ""
    "let res = request::device_attributes(Duration::from_millis(100))?;"
    "term.consume_available()?;"
    "println!(\"{res:#?}\");"
    ""
    "Result::Ok(())"
    "```"
    ""
    "## Result in terminal"
    "![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/\
        assets/raw/request/device_attributes.png)"
    device_attributes = REQUEST_DEVICE_ATTRIBUTES -> TermAttr {
        Event::Status(Status::Attributes(a)) => a
    }

    ? "Requests the cursor position. Result is the type [`Vec2`] with cursor"
    "position in characters. The answer from the terminal is ambiguous. To"
    "avoid this, you can use [`cursor_position2`] which will have unambiguous"
    "response, but it is not supported by some terminals."
    ""
    "Uses the code [`codes::REQUEST_CURSOR_POSITION`]."
    ""
    "This will guard the response with status report and timeout. It will wait"
    "for the expected response. This will return error if response to status"
    "report will be received before the expected response or if the timeout"
    "passes."
    ""
    "Note that the status report is not consumed on success."
    ""
    "# Errors"
    "- [`Error::NoResponse`] if status report is received or if the timeout"
    "  passes."
    "- [`Error::Io`] on io error when working with stdin and stdout."
    "- [`Error::NotSupportedOnPlatform`] if raw mode is not supported on this"
    "  platform. It is supported on windows and unix (linux)."
    "- [`Error::StdInEof`] when stdin reaches eof."
    "- [`Error::WaitAbandoned`] when fails to wait for stdin. See the error"
    "  documentation for more info."
    ""
    "# Example"
    "```no_run"
    "use std::time::Duration;"
    "use termal_core::{codes, raw::{Terminal, request}, Result};"
    ""
    "let mut term = Terminal::stdio();"
    "term.flushed(codes::CLEAR)?;"
    ""
    "let res = request::cursor_position(Duration::from_millis(100))?;"
    "term.consume_available()?;"
    "println!(\"{res:#?}\");"
    ""
    "Result::Ok(())"
    "```"
    ""
    "## Result in terminal"
    "![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/\
        assets/raw/request/cursor_position.png)"
    cursor_position = REQUEST_CURSOR_POSITION -> Vec2 {
        Event::Status(Status::CursorPosition { x, y }) => (x, y).into()
    }

    ? "Requests the cursor position. Result is the type [`Vec2`] with cursor"
    "position in characters. The response is unambiguous, but it is not"
    "supported by some terminals."
    ""
    "Uses the code [`codes::REQUEST_CURSOR_POSITION2`]."
    ""
    "This will guard the response with status report and timeout. It will wait"
    "for the expected response. This will return error if response to status"
    "report will be received before the expected response or if the timeout"
    "passes."
    ""
    "Note that the status report is not consumed on success."
    ""
    "# Errors"
    "- [`Error::NoResponse`] if status report is received or if the timeout"
    "  passes."
    "- [`Error::Io`] on io error when working with stdin and stdout."
    "- [`Error::NotSupportedOnPlatform`] if raw mode is not supported on this"
    "  platform. It is supported on windows and unix (linux)."
    "- [`Error::StdInEof`] when stdin reaches eof."
    "- [`Error::WaitAbandoned`] when fails to wait for stdin. See the error"
    "  documentation for more info."
    ""
    "# Example"
    "```no_run"
    "use std::time::Duration;"
    "use termal_core::{codes, raw::{Terminal, request}, Result};"
    ""
    "let mut term = Terminal::stdio();"
    "term.flushed(codes::CLEAR)?;"
    ""
    "let res = request::cursor_position2(Duration::from_millis(100))?;"
    "term.consume_available()?;"
    "println!(\"{res:#?}\");"
    ""
    "Result::Ok(())"
    "```"
    ""
    "## Result in terminal"
    "![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/\
        assets/raw/request/cursor_position.png)"
    cursor_position2 = REQUEST_CURSOR_POSITION2 -> Vec2 {
        Event::Status(Status::CursorPosition { x, y }) => (x, y).into()
    }

    ? "Requests the terminal name."
    ""
    "Uses the code [`codes::REQUEST_TERMINAL_NAME`]."
    ""
    "This will guard the response with status report and timeout. It will wait"
    "for the expected response. This will return error if response to status"
    "report will be received before the expected response or if the timeout"
    "passes."
    ""
    "Note that the status report is not consumed on success."
    ""
    "# Errors"
    "- [`Error::NoResponse`] if status report is received or if the timeout"
    "  passes."
    "- [`Error::Io`] on io error when working with stdin and stdout."
    "- [`Error::NotSupportedOnPlatform`] if raw mode is not supported on this"
    "  platform. It is supported on windows and unix (linux)."
    "- [`Error::StdInEof`] when stdin reaches eof."
    "- [`Error::WaitAbandoned`] when fails to wait for stdin. See the error"
    "  documentation for more info."
    ""
    "# Example"
    "```no_run"
    "use std::time::Duration;"
    "use termal_core::{codes, raw::{Terminal, request}, Result};"
    ""
    "let mut term = Terminal::stdio();"
    "term.flushed(codes::CLEAR)?;"
    ""
    "let res = request::terminal_name(Duration::from_millis(100))?;"
    "term.consume_available()?;"
    "println!(\"{res}\");"
    ""
    "Result::Ok(())"
    "```"
    ""
    "## Result in terminal"
    "![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/\
        assets/raw/request/terminal_name.png)"
    terminal_name = REQUEST_TERMINAL_NAME -> String {
        Event::Status(Status::TerminalName(n)) => n
    }

    ? "Requests size of the text area in pixels."
    ""
    "Uses the code [`codes::REQUEST_TEXT_AREA_SIZE_PX`]."
    ""
    "This will guard the response with status report and timeout. It will wait"
    "for the expected response. This will return error if response to status"
    "report will be received before the expected response or if the timeout"
    "passes."
    ""
    "Note that the status report is not consumed on success."
    ""
    "# Errors"
    "- [`Error::NoResponse`] if status report is received or if the timeout"
    "  passes."
    "- [`Error::Io`] on io error when working with stdin and stdout."
    "- [`Error::NotSupportedOnPlatform`] if raw mode is not supported on this"
    "  platform. It is supported on windows and unix (linux)."
    "- [`Error::StdInEof`] when stdin reaches eof."
    "- [`Error::WaitAbandoned`] when fails to wait for stdin. See the error"
    "  documentation for more info."
    ""
    "# Example"
    "```no_run"
    "use std::time::Duration;"
    "use termal_core::{codes, raw::{Terminal, request}, Result};"
    ""
    "let mut term = Terminal::stdio();"
    "term.flushed(codes::CLEAR)?;"
    ""
    "let res = request::text_area_size_px(Duration::from_millis(100))?;"
    "term.consume_available()?;"
    "println!(\"{res:#?}\");"
    ""
    "Result::Ok(())"
    "```"
    ""
    "## Result in terminal"
    "![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/\
        assets/raw/request/text_area_size_px.png)"
    text_area_size_px = REQUEST_TEXT_AREA_SIZE_PX -> Vec2 {
        Event::Status(Status::TextAreaSizePx { w, h }) => (w, h).into()
    }

    char_size = REQUEST_CHAR_SIZE -> Vec2 {
        Event::Status(Status::CharSize { w, h }) => (w, h).into()
    }

    text_area_size = REQUEST_TEXT_AREA_SIZE -> Vec2 {
        Event::Status(Status::TextAreaSize { w, h }) => (w, h).into()
    }

    sixel_colors = REQUEST_SIXEL_COLORS -> usize {
        Event::Status(Status::SixelColors(n)) => n
    }

    color_code = request_color_code!(code: u8) -> (u8, Rgb<u16>) {
        Event::Status(Status::ColorCodeColor { code, color }) => (code, color),
    }

    default_fg_color = REQUEST_DEFAULT_FG_COLOR -> Rgb<u16> {
        Event::Status(Status::DefaultFgColor(c)) => c
    }

    default_bg_color = REQUEST_DEFAULT_BG_COLOR -> Rgb<u16> {
        Event::Status(Status::DefaultBgColor(c)) => c
    }

    cursor_color = REQUEST_CURSOR_COLOR -> Rgb<u16> {
        Event::Status(Status::CursorColor(c)) => c,
    }

    selection = request_selection(
        sel: impl IntoIterator<Item = Selection>
    ) -> Vec<u8> {
        Event::Status(Status::SelectionData(s)) => s
    }
);

/// Request status report. Returns without error on success (duh :).
///
/// Uses the code [`codes::REQUEST_STATUS_REPORT`].
///
/// This will guard the response with timeout. It will wait for the expected
/// response. This will return error if the timeout passes before receiving
/// response.
///
/// If `req` is true, it will request the status report. If it is false, the
/// status report will not be requested. This may be useful to consume status
/// report.
///
/// # Errors
/// - [`Error::NoResponse`] if the timeout passes.
/// - [`Error::Io`] on io error when working with stdin and stdout.
/// - [`Error::NotSupportedOnPlatform`] if raw mode is not supported on this
///   platform. It is supported on windows and unix (linux).
/// - [`Error::StdInEof`] when stdin reaches eof.
///
/// ## Windows
/// - [`Error::WaitAbandoned`] when fails to wait for stdin. See the error
///   documentation for more info.
///
/// # Example
/// ```no_run
/// use std::time::Duration;
/// use termal_core::{codes, raw::{Terminal, request}, Result};
///
/// let mut term = Terminal::stdio();
/// term.flushed(codes::CLEAR)?;
///
/// let res = request::status_report(Duration::from_millis(100), true)?;
/// println!("{res:#?}");
///
/// Result::Ok(())
/// ```
pub fn status_report(timeout: Duration, req: bool) -> Result<()> {
    raw_guard(true, || {
        let mut term = Terminal::stdio();
        if req {
            term.flushed(codes::REQUEST_STATUS_REPORT)?;
        }

        let mut now = Instant::now();
        let end_time = now + timeout;
        loop {
            let Some(evt) = term.read_timeout(end_time - now)? else {
                return Err(Error::NoResponse);
            };

            if matches!(evt, Event::Status(Status::Ok)) {
                return Ok(());
            }

            now = Instant::now();
            if now >= end_time {
                return Err(Error::NoResponse);
            }
        }
    })
}
