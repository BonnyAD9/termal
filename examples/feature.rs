use std::{
    borrow::Cow,
    env::args,
    io::{IsTerminal, stdout},
};

use features::*;
use termal::{
    eprintacln, error::Result, gradient, printacln, register_reset_on_panic,
};

mod features;

pub fn main() -> Result<()> {
    if let Some(a) = args().nth(1) {
        single(&a)
    } else {
        help()
    }
}

fn single(name: &str) -> Result<()> {
    register_reset_on_panic();
    match name {
        "-h" | "-?" | "--help" => help(),
        "codes::BELL" => codes::show_bell(),
        "codes::BACKSPACE" => codes::show_backspace(),
        "codes::HTAB" => codes::show_htab(),
        "codes::NEWLINE" => codes::show_newline(),
        "codes::VTAB" => codes::show_vtab(),
        "codes::FORMFEED" => codes::show_formfeed(),
        "codes::CARRIAGE_RETURN" => codes::show_carriage_return(),
        "codes::DELETE" => codes::show_delete(),
        "codes::move_to!" => codes::show_move_to(),
        "codes::move_up!" => codes::show_move_up_down(),
        "codes::move_down!" => codes::show_move_up_down(),
        "codes::move_right!" => codes::show_move_right_left(),
        "codes::move_left!" => codes::show_move_right_left(),
        "codes::insert_lines!" => codes::show_insert_lines(),
        "codes::delete_lines!" => codes::show_delete_lines(),
        "codes::insert_chars!" => codes::show_insert_chars(),
        "codes::delete_chars!" => codes::show_delete_chars(),
        "codes::insert_columns!" => codes::show_insert_columns(),
        "codes::delete_columns!" => codes::show_delete_columns(),
        "codes::set_down!" => codes::show_set_down(),
        "codes::set_up!" => codes::show_set_up(),
        "codes::repeat_char!" => codes::show_repeat_char(),
        "codes::column!" => codes::show_column(),
        "codes::MOVE_HOME" => codes::show_move_home(),
        "codes::UP_SCRL" => codes::show_up_scrl(),
        "codes::CUR_SAVE" => codes::show_cur_save_load(),
        "codes::CUR_LOAD" => codes::show_cur_save_load(),
        "codes::ERASE_TO_END" => codes::show_erase_to_end(),
        "codes::ERASE_FROM_START" => codes::show_erase_from_start(),
        "codes::ERASE_SCREEN" => codes::show_erase_screen(),
        "codes::ERASE_BUFFER" => codes::show_erase_buffer(),
        "codes::ERASE_TO_LN_END" => codes::show_erase_to_ln_end(),
        "codes::ERASE_FROM_LN_START" => codes::show_erase_from_ln_start(),
        "codes::ERASE_LINE" => codes::show_erase_line(),
        "codes::ERASE_ALL" => codes::show_erase_all(),
        "codes::CLEAR" => codes::show_clear(),
        "codes::RESET" => codes::show_reset(),
        "codes::BOLD" => codes::show_bold(),
        "codes::FAINT" => codes::show_faint(),
        "codes::ITALIC" => codes::show_italic(),
        "codes::UNDERLINE" => codes::show_underline(),
        "codes::BLINKING" => codes::show_blinking(),
        "codes::INVERSE" => codes::show_inverse(),
        "codes::INVISIBLE" => codes::show_invisible(),
        "codes::STRIKETROUGH" => codes::show_strigetrough(),
        "codes::DOUBLE_UNDERLINE" => codes::show_double_underline(),
        "codes::OVERLINE" => codes::show_overline(),
        "codes::RESET_BOLD" => codes::show_reset_bold(),
        "codes::RESET_ITALIC" => codes::show_italic(),
        "codes::RESET_UNDERLINE" => codes::show_reset_underline(),
        "codes::RESET_BLINKING" => codes::show_blinking(),
        "codes::RESET_INVERSE" => codes::show_inverse(),
        "codes::RESET_INVISIBLE" => codes::show_invisible(),
        "codes::RESET_STRIKETROUGH" => codes::show_strigetrough(),
        "codes::RESET_OVERLINE" => codes::show_overline(),
        "codes::BLACK_FG" => codes::show_black_fg(),
        "codes::WHITE_FG" => codes::show_black_fg(),
        "codes::GRAY_FG" => codes::show_gray_fg(),
        "codes::GRAY_BRIGHT_FG" => codes::show_gray_fg(),
        "codes::RED_FG" => codes::show_red_fg(),
        "codes::GREEN_FG" => codes::show_green_fg(),
        "codes::YELLOW_FG" => codes::show_yellow_fg(),
        "codes::BLUE_FG" => codes::show_blue_fg(),
        "codes::MAGENTA_FG" => codes::show_magenta_fg(),
        "codes::CYAN_FG" => codes::show_cyan_fg(),
        "codes::RED_DARK_FG" => codes::show_red_fg(),
        "codes::GREEN_DARK_FG" => codes::show_green_fg(),
        "codes::YELLOW_DARK_FG" => codes::show_yellow_fg(),
        "codes::BLUE_DARK_FG" => codes::show_blue_fg(),
        "codes::MAGENTA_DARK_FG" => codes::show_magenta_fg(),
        "codes::CYAN_DARK_FG" => codes::show_cyan_fg(),
        "codes::RESET_FG" => codes::show_reset_fg(),
        "codes::BLACK_BG" => codes::show_black_bg(),
        "codes::WHITE_BG" => codes::show_black_bg(),
        "codes::GRAY_BG" => codes::show_gray_bg(),
        "codes::GRAY_BRIGHT_BG" => codes::show_gray_bg(),
        "codes::RED_BG" => codes::show_red_bg(),
        "codes::GREEN_BG" => codes::show_green_bg(),
        "codes::YELLOW_BG" => codes::show_yellow_bg(),
        "codes::BLUE_BG" => codes::show_blue_bg(),
        "codes::MAGENTA_BG" => codes::show_magenta_bg(),
        "codes::CYAN_BG" => codes::show_cyan_bg(),
        "codes::RED_DARK_BG" => codes::show_red_bg(),
        "codes::GREEN_DARK_BG" => codes::show_green_bg(),
        "codes::YELLOW_DARK_BG" => codes::show_yellow_bg(),
        "codes::BLUE_DARK_BG" => codes::show_blue_bg(),
        "codes::MAGENTA_DARK_BG" => codes::show_magenta_bg(),
        "codes::CYAN_DARK_BG" => codes::show_cyan_bg(),
        "codes::RESET_BG" => codes::show_reset_bg(),
        "codes::fg256!" => codes::show_fg256(),
        "codes::bg256!" => codes::show_bg256(),
        "codes::underline256!" => codes::show_underline256(),
        "codes::fg!" => codes::show_fg(),
        "codes::bg!" => codes::show_bg(),
        "codes::underline_rgb!" => codes::show_underline_rgb(),
        "codes::RESET_UNDERLINE_COLOR" => codes::show_reset_underline_color(),
        "codes::DOUBLE_CHAR_HEIGHT_DOWN" => {
            codes::show_double_char_height_down()
        }
        "codes::DOUBLE_CHAR_HEIGHT_UP" => codes::show_double_char_height_up(),
        "codes::DOUBLE_CHAR_WIDTH" => codes::show_double_char_width(),
        "codes::RESET_CHAR_SIZE" => codes::show_reset_char_size(),
        "codes::ENABLE_LINE_WRAP" => codes::show_disable_line_wrap(),
        "codes::DISABLE_LINE_WRAP" => codes::show_disable_line_wrap(),
        "codes::ENABLE_REVERSE_COLOR" => codes::show_enable_reverse_color(),
        "codes::DISABLE_REVERSE_COLOR" => codes::show_enable_reverse_color(),
        "codes::HIDE_CURSOR" => codes::show_hide_cursor(),
        "codes::SHOW_CURSOR" => codes::show_hide_cursor(),
        "codes::SAVE_SCREEN" => codes::show_save_screen(),
        "codes::LOAD_SCREEN" => codes::show_save_screen(),
        "codes::ENABLE_ALTERNATIVE_BUFFER" => {
            codes::show_enable_alternative_buffer()
        }
        "codes::DISABLE_ALTERNATIVE_BUFFER" => {
            codes::show_enable_alternative_buffer()
        }
        "codes::FULL_RESET" => codes::show_full_reset(),
        "codes::REQUEST_DEVICE_ATTRIBUTES" => {
            codes::show_request_device_attributes()
        }
        "codes::REQUEST_STATUS_REPORT" => codes::show_request_status_report(),
        "codes::REQUEST_CURSOR_POSITION" => {
            codes::show_request_cursor_position()
        }
        "codes::REQUEST_CURSOR_POSITION2" => {
            codes::show_request_cursor_position2()
        }
        "codes::REQUEST_TERMINAL_NAME" => codes::show_request_terminal_name(),
        "codes::REQUEST_TEXT_AREA_SIZE_PX" => {
            codes::show_request_text_area_size_px()
        }
        "codes::REQUEST_CHAR_SIZE" => codes::show_request_char_size(),
        "codes::REQUEST_TEXT_AREA_SIZE" => {
            codes::show_request_text_area_size()
        }
        "codes::REQUEST_SIXEL_COLORS" => codes::show_request_sixel_colors(),
        "codes::ENABLE_MOUSE_XY_TRACKING" => {
            codes::show_enable_mouse_xy_tracking()
        }
        "codes::ENABLE_MOUSE_XY_PR_TRACKING" => {
            codes::show_enable_mouse_xy_pr_tracking()
        }
        "codes::ENABLE_MOUSE_XY_DRAG_TRACKING" => {
            codes::show_enable_mouse_xy_drag_tracking()
        }
        "codes::ENABLE_MOUSE_XY_ALL_TRACKING" => {
            codes::show_enable_mouse_xy_all_tracking()
        }
        "codes::ENABLE_FOCUS_EVENT" => codes::show_enable_focus_event(),
        "codes::ENABLE_MOUSE_XY_UTF8_EXT" => {
            codes::show_enable_mouse_xy_utf8_ext()
        }
        "codes::ENABLE_MOUSE_XY_EXT" => codes::show_enable_mouse_xy_ext(),
        "codes::ENABLE_MOUSE_XY_URXVT_EXT" => {
            codes::show_enable_mouse_xy_urxvt_ext()
        }
        "codes::ENABLE_MOUSE_XY_PIX_EXT" => {
            codes::show_enable_mouse_xy_pix_ext()
        }
        "codes::scroll_region!" => codes::show_scroll_region(),
        "codes::LIMIT_PRINT_TO_SCROLL_REGION" => {
            codes::show_limit_print_to_scroll_region()
        }
        "codes::ENABLE_BRACKETED_PASTE_MODE" => {
            codes::show_enable_bracketed_paste_mode()
        }
        "codes::DISABLE_BRACKETED_PASTE_MODE" => {
            codes::show_enable_bracketed_paste_mode()
        }
        "codes::set_cursor" => codes::show_set_cursor(),
        "codes::request_color_code!" => codes::show_request_color_code(),
        "codes::define_color_code" => codes::show_define_color_code(),
        "codes::set_default_fg_color" => codes::show_set_default_fg_color(),
        "codes::set_default_bg_color" => codes::show_set_default_bg_color(),
        "codes::set_cursor_color" => codes::show_set_cursor_color(),
        "codes::REQUEST_DEFAULT_FG_COLOR" => {
            codes::show_request_default_fg_color()
        }
        "codes::REQUEST_DEFAULT_BG_COLOR" => {
            codes::show_request_default_bg_color()
        }
        "codes::REQUEST_CURSOR_COLOR" => codes::show_request_cursor_color(),
        "codes::REQUEST_SELECTION" => codes::show_request_selection(),
        "codes::set_selection" => codes::show_set_selection(),
        "write_gradient" => show_write_gradient(),
        "gradient" => show_gradient(),
        _ => {
            eprintacln!("{'r}error: {'_}unknown feature `{name}`.");
            Ok(())
        }
    }
}

fn help() -> Result<()> {
    let sign: Cow<str> = if stdout().is_terminal() {
        gradient("BonnyAD9", (250, 50, 170), (180, 50, 240)).into()
    } else {
        "BonnyAD9".into()
    };

    printacln!(
        "Welcome to examples for the library {'i g}termal by {sign}.

{'g}Usage:
  {'c}feature {'y}--help{'_}
    Show this help.

  {'c}feature {'gr}<library item>{'_}
    Show help for the given library item.

When running with cargo, instead of `{'c}feature{'_}` you use
`{'c}cargo {'b}run {'y}--features {'w}all {'y}--example {'w}feature \
{'y}--{'_}`.

{'g}Available library items:
  {'c}codes{'y}::{'w bold}BELL{'_}
  {'c}codes{'y}::{'w bold}BACKSPACE{'_}
  {'c}codes{'y}::{'w bold}HTAB{'_}
  {'c}codes{'y}::{'w bold}NEWLINE{'_}
  {'c}codes{'y}::{'w bold}VTAB{'_}
  {'c}codes{'y}::{'w bold}FORMFEED{'_}
  {'c}codes{'y}::{'w bold}CARRIAGE_RETURN{'_}
  {'c}codes{'y}::{'w bold}DELETE{'_}
  {'c}codes{'y}::{'m i}move_to!{'_}
  {'c}codes{'y}::{'m i}move_up!{'_}
  {'c}codes{'y}::{'m i}move_down!{'_}
  {'c}codes{'y}::{'m i}move_right!{'_}
  {'c}codes{'y}::{'m i}move_left!{'_}
  {'c}codes{'y}::{'m i}insert_lines!{'_}
  {'c}codes{'y}::{'m i}delete_lines!{'_}
  {'c}codes{'y}::{'m i}insert_chars!{'_}
  {'c}codes{'y}::{'m i}delete_chars!{'_}
  {'c}codes{'y}::{'m i}insert_colums!{'_}
  {'c}codes{'y}::{'m i}delete_columns!{'_}
  {'c}codes{'y}::{'m i}set_down!{'_}
  {'c}codes{'y}::{'m i}set_up!{'_}
  {'c}codes{'y}::{'m i}repeat_char!{'_}
  {'c}codes{'y}::{'m i}column!{'_}
  {'c}codes{'y}::{'w bold}MOVE_HOME{'_}
  {'c}codes{'y}::{'w bold}UP_SCRL{'_}
  {'c}codes{'y}::{'w bold}CUR_SAVE{'_}
  {'c}codes{'y}::{'w bold}CUR_LOAD{'_}
  {'c}codes{'y}::{'w bold}ERASE_TO_END{'_}
  {'c}codes{'y}::{'w bold}ERASE_FROM_START{'_}
  {'c}codes{'y}::{'w bold}ERASE_SCREEN{'_}
  {'c}codes{'y}::{'w bold}ERASE_BUFFER{'_}
  {'c}codes{'y}::{'w bold}ERASE_TO_LN_END{'_}
  {'c}codes{'y}::{'w bold}ERASE_FROM_LN_START{'_}
  {'c}codes{'y}::{'w bold}ERASE_LINE{'_}
  {'c}codes{'y}::{'w bold}ERASE_ALL{'_}
  {'c}codes{'y}::{'w bold}CLEAR{'_}
  {'c}codes{'y}::{'w bold}RESET{'_}
  {'c}codes{'y}::{'w bold}BOLD{'_}
  {'c}codes{'y}::{'w bold}FAINT{'_}
  {'c}codes{'y}::{'w bold}ITALIC{'_}
  {'c}codes{'y}::{'w bold}UNDERLINE{'_}
  {'c}codes{'y}::{'w bold}BLINKING{'_}
  {'c}codes{'y}::{'w bold}INVERSE{'_}
  {'c}codes{'y}::{'w bold}INVISIBLE{'_}
  {'c}codes{'y}::{'w bold}STRIKETROUGH{'_}
  {'c}codes{'y}::{'w bold}DOUBLE_UNDERLINE{'_}
  {'c}codes{'y}::{'w bold}OVERLINE{'_}
  {'c}codes{'y}::{'w bold}RESET_BOLD{'_}
  {'c}codes{'y}::{'w bold}RESET_ITALIC{'_}
  {'c}codes{'y}::{'w bold}RESET_UNDERLINE{'_}
  {'c}codes{'y}::{'w bold}RESET_BLINKING{'_}
  {'c}codes{'y}::{'w bold}RESET_INVERSE{'_}
  {'c}codes{'y}::{'w bold}RESET_INVISIBLE{'_}
  {'c}codes{'y}::{'w bold}RESET_STRIKETROUGH{'_}
  {'c}codes{'y}::{'w bold}RESET_OVERLINE{'_}
  {'c}codes{'y}::{'w bold}BLACK_FG{'_}
  {'c}codes{'y}::{'w bold}WHITE_FG{'_}
  {'c}codes{'y}::{'w bold}GRAY_FG{'_}
  {'c}codes{'y}::{'w bold}GRAY_BRIGHT_FG{'_}
  {'c}codes{'y}::{'w bold}RED_FG{'_}
  {'c}codes{'y}::{'w bold}GREEN_FG{'_}
  {'c}codes{'y}::{'w bold}YELLOW_FG{'_}
  {'c}codes{'y}::{'w bold}BLUE_FG{'_}
  {'c}codes{'y}::{'w bold}MAGENTA_FG{'_}
  {'c}codes{'y}::{'w bold}CYAN_FG{'_}
  {'c}codes{'y}::{'w bold}RED_DARK_FG{'_}
  {'c}codes{'y}::{'w bold}GREEN_DARK_FG{'_}
  {'c}codes{'y}::{'w bold}YELLOW_DARK_FG{'_}
  {'c}codes{'y}::{'w bold}BLUE_DARK_FG{'_}
  {'c}codes{'y}::{'w bold}MAGENTA_DARK_FG{'_}
  {'c}codes{'y}::{'w bold}CYAN_DARK_FG{'_}
  {'c}codes{'y}::{'w bold}RESET_FG{'_}
  {'c}codes{'y}::{'w bold}BLACK_BG{'_}
  {'c}codes{'y}::{'w bold}WHITE_BG{'_}
  {'c}codes{'y}::{'w bold}GRAY_BG{'_}
  {'c}codes{'y}::{'w bold}GRAY_BRIGHT_BG{'_}
  {'c}codes{'y}::{'w bold}RED_BG{'_}
  {'c}codes{'y}::{'w bold}GREEN_BG{'_}
  {'c}codes{'y}::{'w bold}YELLOW_BG{'_}
  {'c}codes{'y}::{'w bold}BLUE_BG{'_}
  {'c}codes{'y}::{'w bold}MAGENTA_BG{'_}
  {'c}codes{'y}::{'w bold}CYAN_BG{'_}
  {'c}codes{'y}::{'w bold}RED_DARK_BG{'_}
  {'c}codes{'y}::{'w bold}GREEN_DARK_BG{'_}
  {'c}codes{'y}::{'w bold}YELLOW_DARK_BG{'_}
  {'c}codes{'y}::{'w bold}BLUE_DARK_BG{'_}
  {'c}codes{'y}::{'w bold}MAGENTA_DARK_BG{'_}
  {'c}codes{'y}::{'w bold}CYAN_DARK_BG{'_}
  {'c}codes{'y}::{'w bold}RESET_BG{'_}
  {'c}codes{'y}::{'m i}fg256!{'_}
  {'c}codes{'y}::{'m i}bg256!{'_}
  {'c}codes{'y}::{'m i}underline256!{'_}
  {'c}codes{'y}::{'m i}fg!{'_}
  {'c}codes{'y}::{'m i}bg!{'_}
  {'c}codes{'y}::{'m i}underline_rgb!{'_}
  {'c}codes{'y}::{'w bold}RESET_UNDERLINE_COLOR{'_}
  {'c}codes{'y}::{'w bold}DOUBLE_CHAR_HEIGHT_DOWN{'_}
  {'c}codes{'y}::{'w bold}DOUBLE_CHAR_HEIGHT_UP{'_}
  {'c}codes{'y}::{'w bold}DOUBLE_CHAR_WIDTH{'_}
  {'c}codes{'y}::{'w bold}RESET_CHAR_SIZE{'_}
  {'c}codes{'y}::{'w bold}ENABLE_LINE_WRAP{'_}
  {'c}codes{'y}::{'w bold}DISABLE_LINE_WRAP{'_}
  {'c}codes{'y}::{'w bold}ENABLE_REVERSE_COLOR{'_}
  {'c}codes{'y}::{'w bold}DISABLE_REVERSE_COLOR{'_}
  {'c}codes{'y}::{'w bold}HIDE_CURSOR{'_}
  {'c}codes{'y}::{'w bold}SHOW_CURSOR{'_}
  {'c}codes{'y}::{'w bold}SAVE_SCREEN{'_}
  {'c}codes{'y}::{'w bold}LOAD_SCREEN{'_}
  {'c}codes{'y}::{'w bold}ENABLE_ALTERNATIVE_BUFFER{'_}
  {'c}codes{'y}::{'w bold}DISABLE_ALTERNATIVE_BUFFER{'_}
  {'c}codes{'y}::{'w bold}FULL_RESET{'_}
  {'c}codes{'y}::{'w bold}REQUEST_DEVICE_ATTRIBUTES{'_}
  {'c}codes{'y}::{'w bold}REQUEST_STATUS_REPORT{'_}
  {'c}codes{'y}::{'w bold}REQUEST_CURSOR_POSITION{'_}
  {'c}codes{'y}::{'w bold}REQUEST_CURSOR_POSITION2{'_}
  {'c}codes{'y}::{'w bold}REQUEST_TERMINAL_NAME{'_}
  {'c}codes{'y}::{'w bold}REQUEST_TEXT_AREA_SIZE_PX{'_}
  {'c}codes{'y}::{'w bold}REQUEST_CHAR_SIZE{'_}
  {'c}codes{'y}::{'w bold}REQUEST_TEXT_AREA_SIZE{'_}
  {'c}codes{'y}::{'w bold}REQUEST_SIXEL_COLORS{'_}
  {'c}codes{'y}::{'w bold}ENABLE_MOUSE_XY_TRACKING{'_}
  {'c}codes{'y}::{'w bold}ENABLE_MOUSE_XY_PR_TRACKING{'_}
  {'c}codes{'y}::{'w bold}ENABLE_MOUSE_XY_DRAG_TRACKING{'_}
  {'c}codes{'y}::{'w bold}ENABLE_MOUSE_XY_ALL_TRACKING{'_}
  {'c}codes{'y}::{'w bold}ENABLE_FOCUS_EVENT{'_}
  {'c}codes{'y}::{'w bold}ENABLE_MOUSE_XY_UTF8_EXT{'_}
  {'c}codes{'y}::{'w bold}ENABLE_MOUSE_XY_EXT{'_}
  {'c}codes{'y}::{'w bold}ENABLE_MOUSE_XY_URXVT_EXT{'_}
  {'c}codes{'y}::{'w bold}ENABLE_MOUSE_XY_PIX_EXT{'_}
  {'c}codes{'y}::{'m}scroll_region!{'_}
  {'c}codes{'y}::{'w bold}LIMIT_PRINT_TO_SCROLL_REGION{'_}
  {'c}codes{'y}::{'w bold}ENABLE_BRACKETED_PASTE_MODE{'_}
  {'c}codes{'y}::{'w bold}DISABLE_BRACKETED_PASTE_MODE{'_}
  {'c}codes{'y}::{'w i}set_cursor{'_}
  {'c}codes{'y}::{'m i}request_color_code!{'_}
  {'c}codes{'y}::{'w i}define_color_code{'_}
  {'c}codes{'y}::{'w i}set_default_fg_color{'_}
  {'c}codes{'y}::{'w i}set_default_bg_color{'_}
  {'c}codes{'y}::{'w i}set_cursor_color{'_}
  {'c}codes{'y}::{'w bold}REQUEST_DEFAULT_FG_COLOR{'_}
  {'c}codes{'y}::{'w bold}REQUEST_DEFAULT_BG_COLOR{'_}
  {'c}codes{'y}::{'w bold}REQUEST_CURSOR_COLOR{'_}
  {'c}codes{'y}::{'w bold}REQUEST_SELECTION{'_}
  {'c}codes{'y}::{'w i}set_selection{'_}
  {'w i}write_gradient{'_}
  {'w i}gradient{'_}
    "
    );
    Ok(())
}
