use std::io::Write;

use termal::{
    codes,
    error::Result,
    formatc, printcln,
    raw::{TermSize, Terminal, enable_raw_mode, term_size},
    reset_terminal,
};

pub fn show_bell() -> Result<()> {
    enable_raw_mode()?;
    println!("{}", codes::BELL);
    reset_terminal();
    Ok(())
}

pub fn show_backspace() -> Result<()> {
    let mut buf = String::new();

    buf += "Some test";
    buf.push(codes::BACKSPACE);
    buf.push(codes::BACKSPACE);
    buf += "x";

    println!("{buf}");

    Ok(())
}

pub fn show_htab() -> Result<()> {
    println!("1\t: number");
    println!("hello\t: greeting");
    Ok(())
}

pub fn show_newline() -> Result<()> {
    println!("normal:");
    println!("one\ntwo");

    println!("raw:");
    enable_raw_mode()?;
    println!("one\ntwo\r");

    reset_terminal();

    Ok(())
}

pub fn show_vtab() -> Result<()> {
    let mut buf = String::new();

    buf += "hello";
    buf.push(codes::VTAB);
    buf += "there";

    println!("{buf}");

    Ok(())
}

pub fn show_formfeed() -> Result<()> {
    let mut buf = String::new();

    buf += "hello";
    buf.push(codes::FORMFEED);
    buf += "there";

    println!("{buf}");

    Ok(())
}

pub fn show_carriage_return() -> Result<()> {
    println!("hello me\rgreet");
    Ok(())
}

pub fn show_delete() -> Result<()> {
    let mut buf = String::new();

    buf += "hello";
    buf.push(codes::BACKSPACE);
    buf.push(codes::DELETE);

    println!("{buf}");

    Ok(())
}

pub fn show_move_to() -> Result<()> {
    let mut buf = String::new();
    buf += codes::ERASE_ALL;

    let txt = "centered";
    let size = term_size()?;
    let x = (size.char_width - txt.len() + 1) / 2;
    let y = size.char_height / 2;
    // If one of arguments is not literal, produces string.
    let center: String = codes::move_to!(x, y);
    buf += &center;
    buf += txt;

    // With literals, it constructs static slice.
    let home: &'static str = codes::move_to!(1, 1);
    buf += home;
    buf += "top left";

    // Move to the second to last line from bottom.
    buf += &codes::move_to!(0, size.char_height - 1);

    println!("{}", buf);

    Ok(())
}

pub fn show_move_up_down() -> Result<()> {
    assert_eq!(formatc!("{'mu5}"), codes::move_up!(5));
    assert_eq!(formatc!("{'md5}"), codes::move_down!(5));
    assert_eq!(formatc!("{'mu}"), codes::move_up!(1));
    assert_eq!(formatc!("{'md}"), codes::move_down!(1));

    printcln!("{'clear}\n\nhello{'mu2}up{'md}down{'md}");
    Ok(())
}

pub fn show_move_right_left() -> Result<()> {
    assert_eq!(formatc!("{'mr5}"), codes::move_right!(5));
    assert_eq!(formatc!("{'ml5}"), codes::move_left!(5));

    printcln!("{'clear}{'mr7}there{'ml11}hello");
    Ok(())
}

pub fn show_insert_lines() -> Result<()> {
    let mut buf = formatc!("{'clear}");

    buf += "line 1\n";
    buf += "line 2\n";
    buf += codes::move_up!(1);
    buf += codes::insert_lines!(2);
    buf += "inserted 1\n";
    buf += "inserted 2\n";

    println!("{buf}");

    Ok(())
}

pub fn show_delete_lines() -> Result<()> {
    let mut buf = formatc!("{'clear}");

    buf += "line 1\n";
    buf += "line 2\n";
    buf += "line 3\n";
    buf += "line 4";
    buf += codes::move_up!(2);
    buf += codes::delete_lines!(2);

    println!("{buf}");

    Ok(())
}

pub fn show_insert_chars() -> Result<()> {
    let mut buf = formatc!("{'clear}");

    buf += "say there";
    buf += codes::move_left!(5);
    buf += codes::insert_chars!(6);
    buf += "hello";

    println!("{buf}");

    Ok(())
}

pub fn show_delete_chars() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += "say hello there";
    buf += codes::move_left!(11);
    buf += codes::delete_chars!(6);

    println!("{buf}");

    Ok(())
}

pub fn show_insert_columns() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += "say line 1\n";
    buf += "say line 2\n";
    buf += "say line 3";
    buf += codes::move_left!(6);
    buf += codes::insert_columns!(9);
    buf += "hello to ";

    println!("{buf}");

    Ok(())
}

pub fn show_delete_columns() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += "say hello to line 1\n";
    buf += "say greeting line 2\n";
    buf += "say no words line 3";
    buf += codes::move_left!(15);
    buf += codes::delete_columns!(9);

    println!("{buf}");

    Ok(())
}

pub fn show_set_down() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += "line one";
    buf += codes::set_down!(2);
    buf += "line two";

    println!("{buf}");

    Ok(())
}

pub fn show_set_up() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += "\n\n";
    buf += "line one";
    buf += codes::set_up!(2);
    buf += "line two";
    buf += "\n\n";

    println!("{buf}");
    Ok(())
}

pub fn show_repeat_char() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += "lo";
    buf += codes::repeat_char!(69);
    buf += "ng word";

    println!("{buf}");

    Ok(())
}

pub fn show_column() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += "hello";
    buf += codes::column!(20);
    buf += "there";

    println!("{buf}");

    Ok(())
}

pub fn show_move_home() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += "\n\nhello below";
    buf += codes::MOVE_HOME;
    buf += "home sweet home\n\n";

    println!("{buf}");

    Ok(())
}

pub fn show_up_scrl() -> Result<()> {
    println!("{}", codes::CLEAR);

    for i in 0..100 {
        print!("\n{i}");
    }

    // Move to the second line on screen.
    let mut buf = codes::MOVE_HOME.to_string();
    buf += codes::move_down!(1);
    // Move up, scrolling is not necesary so it is just move up
    buf += codes::UP_SCRL;
    // Move up, cursor is already on top of the screen, so empty line is
    // inserted. Line at the bottom of the screen is discarded.
    buf += codes::UP_SCRL;

    print!("{buf}");

    _ = Terminal::stdio().flush();

    // Wait for enter. Screenshot is taken before enter is pressed.
    _ = Terminal::stdio().read();

    Ok(())
}

pub fn show_cur_save_load() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += "start";
    buf += codes::CUR_SAVE;
    buf += "\ncontinue here";
    buf += codes::CUR_LOAD;
    buf += " and end here\n";

    println!("{buf}");

    Ok(())
}

pub fn show_erase_to_end() -> Result<()> {
    show_erase(codes::ERASE_TO_END)
}

pub fn show_erase_from_start() -> Result<()> {
    show_erase(codes::ERASE_FROM_START)
}

pub fn show_erase_screen() -> Result<()> {
    show_erase(codes::ERASE_SCREEN)
}

pub fn show_erase_buffer() -> Result<()> {
    show_erase(codes::ERASE_BUFFER)
}

pub fn show_erase_to_ln_end() -> Result<()> {
    show_erase(codes::ERASE_TO_LN_END)
}

pub fn show_erase_from_ln_start() -> Result<()> {
    show_erase(codes::ERASE_FROM_LN_START)
}

pub fn show_erase_line() -> Result<()> {
    show_erase(codes::ERASE_LINE)
}

pub fn show_erase_all() -> Result<()> {
    show_erase(codes::ERASE_ALL)
}

pub fn show_clear() -> Result<()> {
    show_erase(codes::CLEAR)
}

fn show_erase(code: &str) -> Result<()> {
    // Fill the terminal with `#` and move to the center.
    let TermSize {
        char_width: w,
        char_height: h,
        ..
    } = term_size()?;
    let mut buf = "#".to_string() + &codes::repeat_char!(w * h - 1);
    buf += &codes::move_to!(w / 2, h / 2);

    // Use the erase command
    buf += code;

    // Print to the output and wait for enter. Screenshot is taken before enter
    // is pressed.
    Terminal::stdio().flushed(buf)?;
    Terminal::stdio().read()?;

    Ok(())
}

pub fn show_reset() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    // Produce some crazy style for the text.
    buf += codes::BOLD;
    buf += codes::ITALIC;
    buf += codes::OVERLINE;
    buf += codes::DOUBLE_UNDERLINE;
    buf += codes::STRIKETROUGH;
    buf += codes::BLUE_FG;
    buf += codes::YELLOW_BG;
    buf += codes::underline256!(1);

    // Text with crazy style
    buf += "crazy style";
    // Reset the text style
    buf += codes::RESET;
    // Write text with normal color
    buf += " normal style";

    println!("{buf}");

    Ok(())
}

pub fn show_bold() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::BOLD;
    buf += "bold text";

    buf += codes::RESET_BOLD;
    buf += " normal text";

    println!("{buf}");

    Ok(())
}

pub fn show_faint() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    let cols = [
        "", // default text color
        codes::GRAY_FG,
        codes::WHITE_FG,
        codes::RED_FG,
        codes::GREEN_FG,
        codes::YELLOW_FG,
        codes::BLUE_FG,
        codes::MAGENTA_FG,
        codes::CYAN_FG,
    ];

    for c in cols {
        buf += c;
        buf += codes::FAINT;
        buf += "faint text";
        buf += codes::RESET_BOLD;
        buf += " normal text\n";
    }

    buf.pop(); // remove the last newline
    buf += codes::RESET_FG;

    println!("{buf}");

    Ok(())
}
