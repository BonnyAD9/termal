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
        $fname:ident = $cname:ident
            $(!($($argm:ident : $atm:ty),* $(,)?))?
            $( ($($argf:ident : $atf:ty),* $(,)?))?
        -> $tname:ty { $mat:pat => $n:expr $(,)? } $(,)?
    )*) => {
        $(pub fn $fname(
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
        })*
    };
}

impl_request!(
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
