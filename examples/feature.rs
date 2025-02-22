use std::env::args;

use features::codes;
use termal::{eprintacln, error::Result};

mod features;

pub fn main() -> Result<()> {
    if let Some(a) = args().nth(1) {
        single(&a)
    } else {
        all()
    }
}

fn single(name: &str) -> Result<()> {
    match name {
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
        _ => {
            eprintacln!("{'r}error: {'_}unknown feature `{name}`.");
            Ok(())
        }
    }
}

fn all() -> Result<()> {
    codes::show_bell()?;
    codes::show_backspace()?;
    codes::show_htab()?;
    codes::show_newline()?;
    codes::show_vtab()?;
    codes::show_formfeed()?;
    codes::show_carriage_return()?;
    codes::show_delete()?;
    codes::show_move_to()?;
    codes::show_move_up_down()?;
    codes::show_move_right_left()?;
    codes::show_insert_lines()?;
    codes::show_delete_lines()?;
    codes::show_insert_chars()?;
    codes::show_delete_chars()?;
    codes::show_insert_columns()?;
    codes::show_delete_columns()?;
    codes::show_set_down()?;
    codes::show_set_up()?;
    codes::show_repeat_char()?;
    codes::show_column()?;
    codes::show_move_home()?;
    codes::show_up_scrl()?;
    codes::show_cur_save_load()?;
    codes::show_erase_to_end()?;
    codes::show_erase_from_start()?;
    codes::show_erase_screen()?;
    codes::show_erase_buffer()?;
    Ok(())
}
