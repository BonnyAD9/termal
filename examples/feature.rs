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
    Ok(())
}
