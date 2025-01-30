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
        _ => {
            eprintacln!("{'r}error: {'_}unknown feature `{name}`.");
            Ok(())
        },
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
    Ok(())
}


