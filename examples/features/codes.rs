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

pub fn show_italic() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::ITALIC;
    buf += "italic text";

    buf += codes::RESET_ITALIC;
    buf += " normal text";

    println!("{buf}");

    Ok(())
}

pub fn show_underline() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::UNDERLINE;
    buf += "underline text";

    buf += codes::RESET_UNDERLINE;
    buf += " normal text";

    println!("{buf}");

    Ok(())
}

pub fn show_blinking() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::BLINKING;
    buf += "blinking text";

    buf += codes::RESET_BLINKING;
    buf += " normal text";

    println!("{buf}");

    Ok(())
}

pub fn show_inverse() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::INVERSE;
    buf += "inverse text";

    buf += codes::RESET_INVERSE;
    buf += " normal text";

    println!("{buf}");

    Ok(())
}

pub fn show_invisible() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::INVISIBLE;
    buf += "invisible text";

    buf += codes::RESET_INVISIBLE;
    buf += " normal text";

    println!("{buf}");

    Ok(())
}

pub fn show_strigetrough() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::STRIKETROUGH;
    buf += "striketrough text";

    buf += codes::RESET_STRIKETROUGH;
    buf += " normal text";

    println!("{buf}");

    Ok(())
}

pub fn show_double_underline() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::DOUBLE_UNDERLINE;
    buf += "double underline text";

    buf += codes::RESET_UNDERLINE;
    buf += " normal text";

    println!("{buf}");

    Ok(())
}

pub fn show_overline() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::OVERLINE;
    buf += "overline text";

    buf += codes::RESET_OVERLINE;
    buf += " normal text";

    println!("{buf}");

    Ok(())
}

pub fn show_reset_bold() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::BOLD;
    buf += "bold text";

    buf += codes::RESET_BOLD;
    buf += " normal text\n";

    buf += codes::FAINT;
    buf += "faint text";

    buf += codes::RESET_BOLD;
    buf += " normal text";

    println!("{buf}");

    Ok(())
}

pub fn show_reset_underline() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::UNDERLINE;
    buf += "underline text";

    buf += codes::RESET_UNDERLINE;
    buf += " normal text\n";

    buf += codes::DOUBLE_UNDERLINE;
    buf += "double underline";

    buf += codes::RESET_UNDERLINE;
    buf += " normal text";

    println!("{buf}");

    Ok(())
}

pub fn show_black_fg() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += "normal";
    buf += codes::BLACK_FG;
    buf += " black";
    buf += codes::WHITE_FG;
    buf += " white\n";
    buf += codes::RESET_FG;

    buf += codes::FAINT;
    buf += "faint ";
    buf += codes::BLACK_FG;
    buf += " black";
    buf += codes::WHITE_FG;
    buf += " white";
    buf += codes::RESET;

    println!("{buf}");

    Ok(())
}

pub fn show_gray_fg() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += "normal";
    buf += codes::GRAY_FG;
    buf += " gray";
    buf += codes::GRAY_BRIGHT_FG;
    buf += " bright\n";
    buf += codes::RESET_FG;

    buf += codes::FAINT;
    buf += "faint ";
    buf += codes::GRAY_FG;
    buf += " gray";
    buf += codes::GRAY_BRIGHT_FG;
    buf += " bright";
    buf += codes::RESET;

    println!("{buf}");

    Ok(())
}

pub fn show_red_fg() -> Result<()> {
    show_color_fg("red", codes::RED_FG, codes::RED_DARK_FG)
}

pub fn show_green_fg() -> Result<()> {
    show_color_fg("green", codes::GREEN_FG, codes::GREEN_DARK_FG)
}

pub fn show_yellow_fg() -> Result<()> {
    show_color_fg("yellow", codes::YELLOW_FG, codes::YELLOW_DARK_FG)
}

pub fn show_blue_fg() -> Result<()> {
    show_color_fg("blue", codes::BLUE_FG, codes::BLUE_DARK_FG)
}

pub fn show_magenta_fg() -> Result<()> {
    show_color_fg("magenta", codes::MAGENTA_FG, codes::MAGENTA_DARK_FG)
}

pub fn show_cyan_fg() -> Result<()> {
    show_color_fg("cyan", codes::CYAN_FG, codes::CYAN_DARK_FG)
}

pub fn show_color_fg(n: &str, b: &str, d: &str) -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += "normal";
    buf += b;
    buf += " ";
    buf += n;
    buf += d;
    buf += " dark\n";
    buf += codes::RESET_FG;

    buf += codes::FAINT;
    buf += "faint ";
    buf += b;
    buf += " ";
    buf += n;
    buf += d;
    buf += " dark";
    buf += codes::RESET;

    println!("{buf}");

    Ok(())
}

pub fn show_reset_fg() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::GRAY_BG;
    buf += codes::YELLOW_FG;
    buf += "fg and bg";
    buf += codes::RESET_FG;
    buf += " bg only";
    buf += codes::RESET;

    println!("{buf}");

    Ok(())
}

pub fn show_black_bg() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::BLACK_BG;
    buf += "black";
    buf += codes::WHITE_BG;
    buf += " white";
    buf += codes::RESET_BG;
    buf += " normal";

    println!("{buf}");

    Ok(())
}

pub fn show_gray_bg() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::GRAY_BG;
    buf += "gray";
    buf += codes::GRAY_BRIGHT_BG;
    buf += " bright";
    buf += codes::RESET_BG;
    buf += " normal";

    println!("{buf}");

    Ok(())
}

pub fn show_red_bg() -> Result<()> {
    show_color_bg("red", codes::RED_BG, codes::RED_DARK_BG)
}

pub fn show_green_bg() -> Result<()> {
    show_color_bg("green", codes::GREEN_BG, codes::GREEN_DARK_BG)
}

pub fn show_yellow_bg() -> Result<()> {
    show_color_bg("yellow", codes::YELLOW_BG, codes::YELLOW_DARK_BG)
}

pub fn show_blue_bg() -> Result<()> {
    show_color_bg("blue", codes::BLUE_BG, codes::BLUE_DARK_BG)
}

pub fn show_magenta_bg() -> Result<()> {
    show_color_bg("magenta", codes::MAGENTA_BG, codes::MAGENTA_DARK_BG)
}

pub fn show_cyan_bg() -> Result<()> {
    show_color_bg("cyan", codes::CYAN_BG, codes::CYAN_DARK_BG)
}

fn show_color_bg(n: &str, l: &str, d: &str) -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += l;
    buf += n;
    buf += d;
    buf += " dark";
    buf += codes::RESET_BG;
    buf += " normal";

    println!("{buf}");

    Ok(())
}

pub fn show_reset_bg() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::GRAY_BG;
    buf += codes::YELLOW_FG;
    buf += "fg and bg";
    buf += codes::RESET_BG;
    buf += " fg only";
    buf += codes::RESET;

    println!("{buf}");

    Ok(())
}

pub fn show_fg256() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    for y in 0..16 {
        for x in 0..16 {
            let c = y * 16 + x;

            buf += &codes::fg256!(c);
            buf += &format!("{c:03} ");
        }
        buf.push('\n');
    }

    buf += codes::RESET_FG;

    print!("{buf}");

    Ok(())
}

pub fn show_bg256() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    for y in 0..16 {
        for x in 0..16 {
            let c = y * 16 + x;

            buf += &codes::bg256!(c);
            buf += &format!("{c:03} ");
        }
        buf += codes::RESET_BG;
        buf.push('\n');
    }

    print!("{buf}");

    Ok(())
}

pub fn show_underline256() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();
    const ULS: &[&str] = &[codes::UNDERLINE, codes::DOUBLE_UNDERLINE];

    for y in 0..16 {
        buf += ULS[y % ULS.len()];
        for x in 0..16 {
            let c = y * 16 + x;

            buf += &codes::underline256!(c);
            buf += &format!("{c:03} ");
        }
        buf += codes::RESET_UNDERLINE;
        buf.push('\n');
    }

    print!("{buf}");

    Ok(())
}

pub fn show_fg() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();
    let size = term_size()?;
    let w = size.char_width;
    let h = size.char_height - 1;
    let l = (w * h).isqrt();

    for y in 0..h {
        for x in 0..w {
            let r = y * 256 / h;
            let g = x * 256 / w;
            let b = 255 - (x * y).isqrt() * 256 / l;

            buf += &codes::fg!(r, g, b);
            buf.push('H');
        }
        buf.push('\n');
    }

    print!("{buf}");

    Ok(())
}

pub fn show_bg() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();
    let size = term_size()?;
    let w = size.char_width;
    let h = size.char_height - 1;
    let l = (w * h).isqrt();

    for y in 0..h {
        for x in 0..w {
            let r = y * 256 / h;
            let g = x * 256 / w;
            let b = 255 - (x * y).isqrt() * 256 / l;

            buf += &codes::bg!(r, g, b);
            buf.push('H');
        }
        buf += codes::RESET_BG;
        buf.push('\n');
    }

    print!("{buf}");

    Ok(())
}

pub fn show_underline_rgb() -> Result<()> {
    const ULS: &[&str] = &[codes::UNDERLINE, codes::DOUBLE_UNDERLINE];

    let mut buf = codes::CLEAR.to_string();
    let size = term_size()?;
    let w = size.char_width;
    let h = size.char_height - 1;
    let l = (w * h).isqrt();

    for y in 0..h {
        for x in 0..w {
            let r = y * 256 / h;
            let g = x * 256 / w;
            let b = 255 - (x * y).isqrt() * 256 / l;

            buf += ULS[y % ULS.len()];
            buf += &codes::underline_rgb!(r, g, b);
            buf.push('H');
        }
        buf += codes::RESET_UNDERLINE;
        buf.push('\n');
    }

    buf += codes::RESET_UNDERLINE_COLOR;
    print!("{buf}");

    Ok(())
}

pub fn show_reset_underline_color() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::UNDERLINE;
    buf += codes::underline256!(2);
    buf += "colored";
    buf += codes::RESET_UNDERLINE_COLOR;
    buf += " default";
    buf += codes::RESET_UNDERLINE;

    println!("{buf}");

    Ok(())
}

pub fn show_double_char_height_down() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += "above\n";
    buf += "double";
    buf += codes::DOUBLE_CHAR_HEIGHT_DOWN;
    buf += "\nbelow";

    println!("{buf}");

    Ok(())
}

pub fn show_double_char_height_up() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += "above\n";
    buf += "double";
    buf += codes::DOUBLE_CHAR_HEIGHT_UP;
    buf += "\nbelow";

    println!("{buf}");

    Ok(())
}

pub fn show_double_char_width() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += "normal\n";
    buf += "double";
    buf += codes::DOUBLE_CHAR_WIDTH;

    println!("{buf}");

    Ok(())
}

pub fn show_reset_char_size() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += "\nbig1";
    buf += codes::DOUBLE_CHAR_HEIGHT_UP;
    buf += "\n\nbig2";
    buf += codes::DOUBLE_CHAR_HEIGHT_UP;
    buf += "\nwide1";
    buf += codes::DOUBLE_CHAR_WIDTH;
    buf += "\nwide2";
    buf += codes::DOUBLE_CHAR_WIDTH;

    buf += codes::move_up!(1);
    buf += codes::RESET_CHAR_SIZE;
    buf += codes::move_up!(3);
    buf += codes::RESET_CHAR_SIZE;

    buf += codes::move_down!(4);

    println!("{buf}");

    Ok(())
}

pub fn show_disable_line_wrap() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    buf += codes::DISABLE_LINE_WRAP;
    buf += "this is some long text that doesn't fit on the line without \
        wrapping\n";
    buf += codes::ENABLE_LINE_WRAP;
    buf += "this is some long text that doesn't fit on the line with wrapping";

    println!("{buf}");

    Ok(())
}
