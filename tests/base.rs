use termal::{codes, formatc, formatmc, gradient, write_gradient};

#[test]
fn test_gradient() {
    let txt = "BonnyAD9";
    let s = (250, 50, 170);
    let e = (180, 50, 240);

    let g = gradient(txt, s, e);
    let v = "\x1b[38;2;250;50;170mB\x1b[38;2;240;50;180mo\
        \x1b[38;2;230;50;190mn\x1b[38;2;220;50;200mn\x1b[38;2;210;50;210my\
        \x1b[38;2;200;50;220mA\x1b[38;2;190;50;230mD\x1b[38;2;180;50;240m9";

    assert_eq!(g, v);

    let mut g2 = String::new();
    write_gradient(&mut g2, txt, txt.chars().count(), s, e);
    assert_eq!(g, g2);
}

#[test]
fn test_formatc() {
    let s = "Hello";
    let num = 4;
    let num2 = 0.5;
    assert_eq!(
        formatc!("{'y}{s} {num}{'_}{}", num2),
        "\x1b[93mHello 4\x1b[0m0.5"
    );
    assert_eq!(formatmc!(false, "{'y}{s} {num}{'_}{}", num2), "Hello 40.5");
}

#[test]
fn test_formatc_codes() {
    // True RGB
    assert_eq!(formatc!("{'#123456}"), codes::fg!(0x12, 0x34, 0x56));
    assert_eq!(formatc!("{'#123}"), formatc!("{'#112233}"));
    assert_eq!(formatc!("{'#12}"), formatc!("{'#121212}"));
    assert_eq!(formatc!("{'#1}"), formatc!("{'#111111}"));

    assert_eq!(formatc!("{'#123456_}"), codes::bg!(0x12, 0x34, 0x56));
    assert_eq!(formatc!("{'#123_}"), formatc!("{'#112233_}"));
    assert_eq!(formatc!("{'#12_}"), formatc!("{'#121212_}"));
    assert_eq!(formatc!("{'#1_}"), formatc!("{'#111111_}"));

    assert_eq!(
        formatc!("{'#123456u}"),
        codes::underline_rgb!(0x12, 0x34, 0x56)
    );
    assert_eq!(formatc!("{'#123u}"), formatc!("{'#112233u}"));
    assert_eq!(formatc!("{'#12u}"), formatc!("{'#121212u}"));
    assert_eq!(formatc!("{'#1u}"), formatc!("{'#111111u}"));

    // Ascii
    assert_eq!(formatc!("{'bell}"), codes::BELL.to_string());
    assert_eq!(formatc!("{'backspace}"), codes::BACKSPACE.to_string());
    assert_eq!(formatc!("{'tab}"), "\t");
    assert_eq!(formatc!("{'htab}"), "\t");
    assert_eq!(formatc!("{'move_down_scrl}"), "\n");
    assert_eq!(formatc!("{'mds}"), "\n");
    assert_eq!(formatc!("{'newline}"), "\n\r");
    assert_eq!(formatc!("{'nl}"), "\n\r");
    assert_eq!(formatc!("{'vtab}"), codes::VTAB.to_string());
    assert_eq!(formatc!("{'carriage_return}"), "\r");
    assert_eq!(formatc!("{'cr}"), "\r");

    // Moving cursor
    assert_eq!(formatc!("{'move_to5,4}"), codes::move_to!(5, 4));
    assert_eq!(formatc!("{'mt8,1}"), codes::move_to!(8, 1));
    assert_eq!(formatc!("{'move_up5}"), codes::move_up!(5));
    assert_eq!(formatc!("{'mu5}"), codes::move_up!(5));
    assert_eq!(formatc!("{'move_down5}"), codes::move_down!(5));
    assert_eq!(formatc!("{'md5}"), codes::move_down!(5));
    assert_eq!(formatc!("{'move_right5}"), codes::move_right!(5));
    assert_eq!(formatc!("{'mr5}"), codes::move_right!(5));
    assert_eq!(formatc!("{'move_left5}"), codes::move_left!(5));
    assert_eq!(formatc!("{'ml5}"), codes::move_left!(5));
    assert_eq!(formatc!("{'set_down5}"), codes::set_down!(5));
    assert_eq!(formatc!("{'sd5}"), codes::set_down!(5));
    assert_eq!(formatc!("{'set_up5}"), codes::set_up!(5));
    assert_eq!(formatc!("{'su5}"), codes::set_up!(5));
    assert_eq!(formatc!("{'move_to_column5}"), codes::column!(5));
    assert_eq!(formatc!("{'mc5}"), codes::column!(5));

    assert_eq!(formatc!("{'move_up_scrl}"), codes::UP_SCRL);
    assert_eq!(formatc!("{'save_cur}"), codes::CUR_SAVE);
    assert_eq!(formatc!("{'save}"), codes::CUR_SAVE);
    assert_eq!(formatc!("{'s}"), codes::CUR_SAVE);
    assert_eq!(formatc!("{'load_cur}"), codes::CUR_LOAD);
    assert_eq!(formatc!("{'load}"), codes::CUR_LOAD);
    assert_eq!(formatc!("{'l}"), codes::CUR_LOAD);

    // Erase
    assert_eq!(formatc!("{'erase_to_end}"), codes::ERASE_TO_END);
    assert_eq!(formatc!("{'e_}"), codes::ERASE_TO_END);
    assert_eq!(formatc!("{'erase_from_start}"), codes::ERASE_FROM_START);
    assert_eq!(formatc!("{'_e}"), codes::ERASE_FROM_START);
    assert_eq!(formatc!("{'erase_screen}"), codes::ERASE_SCREEN);
    assert_eq!(formatc!("{'_e_}"), codes::ERASE_SCREEN);
    assert_eq!(formatc!("{'erase_all}"), codes::ERASE_ALL);
    assert_eq!(formatc!("{'e}"), codes::ERASE_ALL);
    assert_eq!(formatc!("{'erase_ln_end}"), codes::ERASE_TO_LN_END);
    assert_eq!(formatc!("{'el_}"), codes::ERASE_TO_LN_END);
    assert_eq!(formatc!("{'erase_ln_start}"), codes::ERASE_FROM_LN_START);
    assert_eq!(formatc!("{'_el}"), codes::ERASE_FROM_LN_START);
    assert_eq!(formatc!("{'erase_line}"), codes::ERASE_LINE);
    assert_eq!(formatc!("{'_el_}"), codes::ERASE_LINE);
    assert_eq!(formatc!("{'el}"), codes::ERASE_LINE);

    // Font style
    assert_eq!(formatc!("{'bold}"), codes::BOLD);
    assert_eq!(formatc!("{'faint}"), codes::FAINT);
    assert_eq!(formatc!("{'f}"), codes::FAINT);
    assert_eq!(formatc!("{'italic}"), codes::ITALIC);
    assert_eq!(formatc!("{'i}"), codes::ITALIC);
    assert_eq!(formatc!("{'underline}"), codes::UNDERLINE);
    assert_eq!(formatc!("{'u}"), codes::UNDERLINE);
    assert_eq!(formatc!("{'blinking}"), codes::BLINKING);
    assert_eq!(formatc!("{'blink}"), codes::BLINKING);
    assert_eq!(formatc!("{'inverse}"), codes::INVERSE);
    assert_eq!(formatc!("{'invisible}"), codes::INVISIBLE);
    assert_eq!(formatc!("{'invis}"), codes::INVISIBLE);
    assert_eq!(formatc!("{'striketrough}"), codes::STRIKETROUGH);
    assert_eq!(formatc!("{'strike}"), codes::STRIKETROUGH);
    assert_eq!(formatc!("{'double_underline}"), codes::DOUBLE_UNDERLINE);
    assert_eq!(formatc!("{'dunderline}"), codes::DOUBLE_UNDERLINE);
    assert_eq!(formatc!("{'dun}"), codes::DOUBLE_UNDERLINE);
    assert_eq!(formatc!("{'overline}"), codes::OVERLINE);
    assert_eq!(formatc!("{'ol}"), codes::OVERLINE);

    assert_eq!(formatc!("{'_bold}"), codes::RESET_BOLD);
    assert_eq!(formatc!("{'_italic}"), codes::RESET_ITALIC);
    assert_eq!(formatc!("{'_i}"), codes::RESET_ITALIC);
    assert_eq!(formatc!("{'_underline}"), codes::RESET_UNDERLINE);
    assert_eq!(formatc!("{'_u}"), codes::RESET_UNDERLINE);
    assert_eq!(formatc!("{'_blinking}"), codes::RESET_BLINKING);
    assert_eq!(formatc!("{'_blink}"), codes::RESET_BLINKING);
    assert_eq!(formatc!("{'_inverse}"), codes::RESET_INVERSE);
    assert_eq!(formatc!("{'_invisible}"), codes::RESET_INVISIBLE);
    assert_eq!(formatc!("{'_invis}"), codes::RESET_INVISIBLE);
    assert_eq!(formatc!("{'_striketrough}"), codes::RESET_STRIKETROUGH);
    assert_eq!(formatc!("{'_strike}"), codes::RESET_STRIKETROUGH);
    assert_eq!(formatc!("{'_overline}"), codes::RESET_OVERLINE);
    assert_eq!(formatc!("{'_ol}"), codes::RESET_OVERLINE);

    assert_eq!(formatc!("{'black_fg}"), codes::BLACK_FG);
    assert_eq!(formatc!("{'black}"), codes::BLACK_FG);
    assert_eq!(formatc!("{'bl}"), codes::BLACK_FG);
    assert_eq!(formatc!("{'white_fg}"), codes::WHITE_FG);
    assert_eq!(formatc!("{'white}"), codes::WHITE_FG);
    assert_eq!(formatc!("{'w}"), codes::WHITE_FG);
    assert_eq!(formatc!("{'gray_fg}"), codes::GRAY_FG);
    assert_eq!(formatc!("{'gray}"), codes::GRAY_FG);
    assert_eq!(formatc!("{'gr}"), codes::GRAY_FG);
    assert_eq!(formatc!("{'bright_gray_fg}"), codes::GRAY_BRIGHT_FG);
    assert_eq!(formatc!("{'bgray}"), codes::GRAY_BRIGHT_FG);
    assert_eq!(formatc!("{'bgr}"), codes::GRAY_BRIGHT_FG);

    assert_eq!(formatc!("{'red_fg}"), codes::RED_FG);
    assert_eq!(formatc!("{'red}"), codes::RED_FG);
    assert_eq!(formatc!("{'r}"), codes::RED_FG);
    assert_eq!(formatc!("{'green_fg}"), codes::GREEN_FG);
    assert_eq!(formatc!("{'green}"), codes::GREEN_FG);
    assert_eq!(formatc!("{'g}"), codes::GREEN_FG);
    assert_eq!(formatc!("{'yellow_fg}"), codes::YELLOW_FG);
    assert_eq!(formatc!("{'yellow}"), codes::YELLOW_FG);
    assert_eq!(formatc!("{'y}"), codes::YELLOW_FG);
    assert_eq!(formatc!("{'magenta_fg}"), codes::MAGENTA_FG);
    assert_eq!(formatc!("{'magenta}"), codes::MAGENTA_FG);
    assert_eq!(formatc!("{'m}"), codes::MAGENTA_FG);
    assert_eq!(formatc!("{'cyan_fg}"), codes::CYAN_FG);
    assert_eq!(formatc!("{'cyan}"), codes::CYAN_FG);
    assert_eq!(formatc!("{'c}"), codes::CYAN_FG);

    assert_eq!(formatc!("{'dark_red_fg}"), codes::RED_DARK_FG);
    assert_eq!(formatc!("{'dred}"), codes::RED_DARK_FG);
    assert_eq!(formatc!("{'dr}"), codes::RED_DARK_FG);
    assert_eq!(formatc!("{'dark_green_fg}"), codes::GREEN_DARK_FG);
    assert_eq!(formatc!("{'dgreen}"), codes::GREEN_DARK_FG);
    assert_eq!(formatc!("{'dg}"), codes::GREEN_DARK_FG);
    assert_eq!(formatc!("{'dark_yellow_fg}"), codes::YELLOW_DARK_FG);
    assert_eq!(formatc!("{'dyellow}"), codes::YELLOW_DARK_FG);
    assert_eq!(formatc!("{'dy}"), codes::YELLOW_DARK_FG);
    assert_eq!(formatc!("{'dark_magenta_fg}"), codes::MAGENTA_DARK_FG);
    assert_eq!(formatc!("{'dmagenta}"), codes::MAGENTA_DARK_FG);
    assert_eq!(formatc!("{'dm}"), codes::MAGENTA_DARK_FG);
    assert_eq!(formatc!("{'dark_cyan_fg}"), codes::CYAN_DARK_FG);
    assert_eq!(formatc!("{'dcyan}"), codes::CYAN_DARK_FG);
    assert_eq!(formatc!("{'dc}"), codes::CYAN_DARK_FG);

    assert_eq!(formatc!("{'_fg}"), codes::RESET_FG);

    assert_eq!(formatc!("{'red_bg}"), codes::RED_BG);
    assert_eq!(formatc!("{'redb}"), codes::RED_BG);
    assert_eq!(formatc!("{'rb}"), codes::RED_BG);
    assert_eq!(formatc!("{'green_bg}"), codes::GREEN_BG);
    assert_eq!(formatc!("{'greenb}"), codes::GREEN_BG);
    assert_eq!(formatc!("{'gb}"), codes::GREEN_BG);
    assert_eq!(formatc!("{'yellow_bg}"), codes::YELLOW_BG);
    assert_eq!(formatc!("{'yellowb}"), codes::YELLOW_BG);
    assert_eq!(formatc!("{'yb}"), codes::YELLOW_BG);
    assert_eq!(formatc!("{'magenta_bg}"), codes::MAGENTA_BG);
    assert_eq!(formatc!("{'magentab}"), codes::MAGENTA_BG);
    assert_eq!(formatc!("{'mb}"), codes::MAGENTA_BG);
    assert_eq!(formatc!("{'cyan_bg}"), codes::CYAN_BG);
    assert_eq!(formatc!("{'cyanb}"), codes::CYAN_BG);
    assert_eq!(formatc!("{'cb}"), codes::CYAN_BG);

    assert_eq!(formatc!("{'dark_red_bg}"), codes::RED_DARK_BG);
    assert_eq!(formatc!("{'dredb}"), codes::RED_DARK_BG);
    assert_eq!(formatc!("{'drb}"), codes::RED_DARK_BG);
    assert_eq!(formatc!("{'dark_green_bg}"), codes::GREEN_DARK_BG);
    assert_eq!(formatc!("{'dgreenb}"), codes::GREEN_DARK_BG);
    assert_eq!(formatc!("{'dgb}"), codes::GREEN_DARK_BG);
    assert_eq!(formatc!("{'dark_yellow_bg}"), codes::YELLOW_DARK_BG);
    assert_eq!(formatc!("{'dyellowb}"), codes::YELLOW_DARK_BG);
    assert_eq!(formatc!("{'dyb}"), codes::YELLOW_DARK_BG);
    assert_eq!(formatc!("{'dark_magenta_bg}"), codes::MAGENTA_DARK_BG);
    assert_eq!(formatc!("{'dmagentab}"), codes::MAGENTA_DARK_BG);
    assert_eq!(formatc!("{'dmb}"), codes::MAGENTA_DARK_BG);
    assert_eq!(formatc!("{'dark_cyan_bg}"), codes::CYAN_DARK_BG);
    assert_eq!(formatc!("{'dcyanb}"), codes::CYAN_DARK_BG);
    assert_eq!(formatc!("{'dcb}"), codes::CYAN_DARK_BG);

    assert_eq!(formatc!("{'_bg}"), codes::RESET_BG);
    assert_eq!(formatc!("{'_uc}"), codes::RESET_UNDERLINE_COLOR);
    assert_eq!(formatc!("{'_ucolor}"), codes::RESET_UNDERLINE_COLOR);

    assert_eq!(formatc!("{'fg56}"), codes::fg256!(56));
    assert_eq!(formatc!("{'bg56}"), codes::bg256!(56));
    assert_eq!(formatc!("{'ucolor56}"), codes::underline256!(56));
    assert_eq!(formatc!("{'uc56}"), codes::underline256!(56));

    // Other
    assert_eq!(formatc!("{'line_wrap}"), codes::ENABLE_LINE_WRAP);
    assert_eq!(formatc!("{'wrap}"), codes::ENABLE_LINE_WRAP);
    assert_eq!(formatc!("{'_line_wrap}"), codes::DISABLE_LINE_WRAP);
    assert_eq!(formatc!("{'_wrap}"), codes::DISABLE_LINE_WRAP);

    assert_eq!(formatc!("{'hide_cursor}"), codes::HIDE_CURSOR);
    assert_eq!(formatc!("{'nocur}"), codes::HIDE_CURSOR);
    assert_eq!(formatc!("{'show_cursor}"), codes::SHOW_CURSOR);
    assert_eq!(formatc!("{'_nocur}"), codes::SHOW_CURSOR);
    assert_eq!(formatc!("{'save_screen}"), codes::SAVE_SCREEN);
    assert_eq!(formatc!("{'sscr}"), codes::SAVE_SCREEN);
    assert_eq!(formatc!("{'load_screen}"), codes::LOAD_SCREEN);
    assert_eq!(formatc!("{'lscr}"), codes::LOAD_SCREEN);
    assert_eq!(formatc!("{'alt_buf}"), codes::ENABLE_ALTERNATIVE_BUFFER);
    assert_eq!(formatc!("{'abuf}"), codes::ENABLE_ALTERNATIVE_BUFFER);
    assert_eq!(formatc!("{'_alt_buf}"), codes::DISABLE_ALTERNATIVE_BUFFER);
    assert_eq!(formatc!("{'_abuf}"), codes::DISABLE_ALTERNATIVE_BUFFER);

    assert_eq!(formatc!("{'clear}"), formatc!("{'e mt}"));
    assert_eq!(formatc!("{'cls}"), formatc!("{'e mt}"));
}
