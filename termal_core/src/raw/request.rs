use std::time::Duration;

use minlin::Vec2;

use crate::{
    Error, Result, Rgb,
    codes::{self, Selection},
    raw::{
        events::{Event, Status, TermAttr},
        request,
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

    status_report = REQUEST_STATUS_REPORT -> bool {
        Event::Status(Status::Ok) => true
    }

    cursor_position = REQUEST_CURSOR_POSITION -> Vec2 {
        Event::Status(Status::CursorPosition { x, y }) => (x, y).into()
    }

    cursor_position2 = REQUEST_CURSOR_POSITION2 -> Vec2 {
        Event::Status(Status::CursorPosition { x, y }) => (x, y).into()
    }

    terminal_name = REQUEST_TERMINAL_NAME -> String {
        Event::Status(Status::TerminalName(n)) => n
    }

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
