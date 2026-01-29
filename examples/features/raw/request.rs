use std::time::Duration;

use termal::{
    Result, codes,
    raw::{Terminal, request},
};

pub fn show_device_attributes() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let res = request::device_attributes(Duration::from_millis(100))?;
    term.consume_available()?;
    println!("{res:#?}");
    Ok(())
}

pub fn show_cursor_position() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let res = request::cursor_position(Duration::from_millis(100))?;
    term.consume_available()?;
    println!("{res:#?}");
    Ok(())
}

pub fn show_cursor_position2() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let res = request::cursor_position2(Duration::from_millis(100))?;
    term.consume_available()?;
    println!("{res:#?}");
    Ok(())
}

pub fn show_terminal_name() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let res = request::terminal_name(Duration::from_millis(100))?;
    term.consume_available()?;
    println!("{res}");
    Ok(())
}

pub fn show_text_area_size_px() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let res = request::text_area_size_px(Duration::from_millis(100))?;
    term.consume_available()?;
    println!("{res:#?}");
    Ok(())
}

pub fn show_char_size() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let res = request::char_size(Duration::from_millis(100))?;
    term.consume_available()?;
    println!("{res:#?}");
    Ok(())
}

pub fn show_text_area_size() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let res = request::text_area_size(Duration::from_millis(100))?;
    term.consume_available()?;
    println!("{res:#?}");
    Ok(())
}

pub fn show_sixel_colors() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let res = request::sixel_colors(Duration::from_millis(100))?;
    term.consume_available()?;
    println!("{res}");
    Ok(())
}

pub fn show_color_code() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let res = request::color_code(11, Duration::from_millis(100))?;
    term.consume_available()?;
    println!("{res:#?}");
    Ok(())
}

pub fn show_default_fg_color() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let res = request::default_fg_color(Duration::from_millis(100))?;
    term.consume_available()?;
    println!("{res:#?}");
    Ok(())
}

pub fn show_default_bg_color() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let res = request::default_bg_color(Duration::from_millis(100))?;
    term.consume_available()?;
    println!("{res:#?}");
    Ok(())
}

pub fn show_cursor_color() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let res = request::cursor_color(Duration::from_millis(100))?;
    term.consume_available()?;
    println!("{res:#?}");
    Ok(())
}

pub fn show_selection() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let res = request::selection([], Duration::from_millis(100))?;
    term.consume_available()?;
    println!("{}", String::from_utf8_lossy(&res));
    Ok(())
}

pub fn show_status_report() -> Result<()> {
    let mut term = Terminal::stdio();
    term.flushed(codes::CLEAR)?;

    let res = request::status_report(Duration::from_millis(100), true)?;
    println!("{res:#?}");
    Ok(())
}
