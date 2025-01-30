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
        "codes::BACKSPACE" => codes::show_backspace(),
        "codes::HTAB" => codes::show_htab(),
        "codes::NEWLINE" => codes::show_newline(),
        "codes::VTAB" => codes::show_vtab(),
        _ => {
            eprintacln!("{'r}error: {'_}unknown feature `{name}`.");
            Ok(())
        },
    }
}

fn all() -> Result<()> {
    codes::show_backspace()?;
    codes::show_htab()?;
    codes::show_newline()?;
    codes::show_vtab()?;
    Ok(())
}


