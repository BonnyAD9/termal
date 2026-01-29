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

    ? "Requests the size of character in pixels."
    ""
    "Uses the code [`codes::REQUEST_CHAR_SIZE`]."
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
    "let res = request::char_size(Duration::from_millis(100))?;"
    "term.consume_available()?;"
    "println!(\"{res:#?}\");"
    ""
    "Result::Ok(())"
    "```"
    ""
    "## Result in terminal"
    "![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/\
        assets/raw/request/char_size.png)"
    char_size = REQUEST_CHAR_SIZE -> Vec2 {
        Event::Status(Status::CharSize { w, h }) => (w, h).into()
    }

    ? "Requests size of the text area in characters."
    ""
    "Uses the code [`codes::REQUEST_TEXT_AREA_SIZE`]."
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
    "let res = request::text_area_size(Duration::from_millis(100))?;"
    "term.consume_available()?;"
    "println!(\"{res:#?}\");"
    ""
    "Result::Ok(())"
    "```"
    ""
    "## Result in terminal"
    "![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/\
        assets/raw/request/text_area_size.png)"
    text_area_size = REQUEST_TEXT_AREA_SIZE -> Vec2 {
        Event::Status(Status::TextAreaSize { w, h }) => (w, h).into()
    }

    ? "Requests the maximum size of sixel color palette."
    ""
    "Uses the code [`codes::REQUEST_SIXEL_COLORS`]."
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
    "let res = request::sixel_colors(Duration::from_millis(100))?;"
    "term.consume_available()?;"
    "println!(\"{res:#?}\");"
    ""
    "Result::Ok(())"
    "```"
    ""
    "## Result in terminal"
    "![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/\
        assets/raw/request/sixel_colors.png)"
    sixel_colors = REQUEST_SIXEL_COLORS -> usize {
        Event::Status(Status::SixelColors(n)) => n
    }

    ? "Requests color of the given color code."
    ""
    "The response contains color code and the color with 16 bit components."
    ""
    "Uses the code [`codes::request_color_code!`]."
    ""
    "Colors in range `0..16` corespond to the named colors in order black,"
    "red, green, yellow, blue, magenta, cyan and yellow. `0..8` are the dark"
    "variants and `8..16` are the bright variants."
    ""
    "Colors in range `16..232` (216 color variants) are usually colors of the"
    "form 16 + RGB in base 6. So for example if you want full green, that is"
    "`050` in base 6, in base 10 that is `30` and than we add 16. So the final"
    "number for full green is `46`."
    ""
    "Colors in range `232..256` are usually 24 shades of gray from dark to"
    "bright not including full black and full white. (full black is 16 and"
    "full white is 231)."
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
    "let res = request::color_code(11, Duration::from_millis(100))?;"
    "term.consume_available()?;"
    "println!(\"{res:#?}\");"
    ""
    "Result::Ok(())"
    "```"
    ""
    "## Result in terminal"
    "![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/\
        assets/raw/request/color_code.png)"
    color_code = request_color_code!(code: u8) -> (u8, Rgb<u16>) {
        Event::Status(Status::ColorCodeColor { code, color }) => (code, color),
    }

    ? "Requests the default foreground color."
    ""
    "The response contains color with 16 bit components."
    ""
    "Uses the code [`codes::REQUEST_DEFAULT_FG_COLOR`]."
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
    "let res = request::default_fg_color(Duration::from_millis(100))?;"
    "term.consume_available()?;"
    "println!(\"{res:#?}\");"
    ""
    "Result::Ok(())"
    "```"
    ""
    "## Result in terminal"
    "![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/\
        assets/raw/request/default_fg_color.png)"
    default_fg_color = REQUEST_DEFAULT_FG_COLOR -> Rgb<u16> {
        Event::Status(Status::DefaultFgColor(c)) => c
    }

    ? "Requests the default background color."
    ""
    "The response contains color with 16 bit components."
    ""
    "Uses the code [`codes::REQUEST_DEFAULT_BG_COLOR`]."
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
    "let res = request::default_bg_color(Duration::from_millis(100))?;"
    "term.consume_available()?;"
    "println!(\"{res:#?}\");"
    ""
    "Result::Ok(())"
    "```"
    ""
    "## Result in terminal"
    "![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/\
        assets/raw/request/default_bg_color.png)"
    default_bg_color = REQUEST_DEFAULT_BG_COLOR -> Rgb<u16> {
        Event::Status(Status::DefaultBgColor(c)) => c
    }

    ? "Requests color of the cursor."
    ""
    "The response contains color with 16 bit components."
    ""
    "Uses the code [`codes::REQUEST_CURSOR_COLOR`]."
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
    "let res = request::cursor_color(Duration::from_millis(100))?;"
    "term.consume_available()?;"
    "println!(\"{res:#?}\");"
    ""
    "Result::Ok(())"
    "```"
    ""
    "## Result in terminal"
    "![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/\
        assets/raw/request/cursor_color.png)"
    cursor_color = REQUEST_CURSOR_COLOR -> Rgb<u16> {
        Event::Status(Status::CursorColor(c)) => c,
    }

    ? "Requests for contents of clipboard."
    ""
    "The input iterator specifies clipboards to request. If empty, it will"
    "request the default clipboard."
    ""
    "Note that if the terminal will not respond immidietely and it will ask"
    "for user permission (such as kitty does by default), it will send the"
    "status report before the clipboard contents and thus result in"
    "[`Error::NoResponse`]."
    ""
    "Uses the code [`codes::request_selection`]."
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
    "let res = request::selection([], Duration::from_millis(100))?;"
    "term.consume_available()?;"
    "println!(\"{}\", String::from_utf8_lossy(&res));"
    ""
    "Result::Ok(())"
    "```"
    ""
    "## Result in terminal"
    "![](https://raw.githubusercontent.com/BonnyAD9/termal/refs/heads/master/\
        assets/raw/request/selection.png)"
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
