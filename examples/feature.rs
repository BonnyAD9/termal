use std::{
    borrow::Cow,
    env::args,
    io::{IsTerminal, stdout},
};

use features::codes;
use termal::{eprintacln, error::Result, gradient, printacln};

mod features;

pub fn main() -> Result<()> {
    if let Some(a) = args().nth(1) {
        single(&a)
    } else {
        help()
    }
}

fn single(name: &str) -> Result<()> {
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
        "codes::RESET_ITALIC" => codes::show_italic(),
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
  {'c}codes{'y}::{'w bold}RESET_ITALIC{'_}
    "
    );
    Ok(())
}
