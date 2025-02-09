use termal::{
    codes::{self, CursorStyle, Selection},
    Rgb,
};

#[test]
fn base_macros() {
    let six = 6;

    assert_eq!(termal::seq!(1, 2), "12");
    assert_eq!(termal::seq!(1, 2, 3), "132");
    assert_eq!(termal::seq!(1, 2, 3, 4, 5), "13;4;52");
    assert_eq!(termal::seq!(1, 2, six, 4, 5), "16;4;52");
    assert_eq!(termal::seq!(1, 2, six, 4, 5, six), "16;4;5;62");

    assert_eq!(termal::csi!('L'), "\x1b[L");
    assert_eq!(termal::csi!('L', 1), "\x1b[1L");
    assert_eq!(termal::csi!('L', 1, 2), "\x1b[1;2L");
    assert_eq!(termal::csi!('L', 1, six), "\x1b[1;6L");

    assert_eq!(termal::graphic!(), "\x1b[m");
    assert_eq!(termal::graphic!(1), "\x1b[1m");
    assert_eq!(termal::graphic!(1, 2), "\x1b[1;2m");
    assert_eq!(termal::graphic!(1, six), "\x1b[1;6m");

    assert_eq!(termal::osc!(1), "\x1b]1\x1b\\");
    assert_eq!(termal::osc!(1, 2), "\x1b]1;2\x1b\\");
    assert_eq!(termal::osc!(1, six), "\x1b]1;6\x1b\\");

    assert_eq!(termal::enable!(1), "\x1b[?1h");
    assert_eq!(termal::enable!(six), "\x1b[?6h");

    assert_eq!(termal::disable!(1), "\x1b[?1l");
    assert_eq!(termal::disable!(six), "\x1b[?6l");
}

#[test]
fn macros() {
    let six = 6;

    assert_eq!(codes::move_to!(5, 4), "\x1b[4;5H");
    assert_eq!(codes::move_to!(5, six), "\x1b[6;5H");

    assert_eq!(codes::move_up!(5), "\x1b[5A");
    assert_eq!(codes::move_up!(six), "\x1b[6A");
    assert_eq!(codes::move_up!(0), "");
    assert_eq!(codes::move_down!(5), "\x1b[5B");
    assert_eq!(codes::move_down!(0), "");
    assert_eq!(codes::move_right!(5), "\x1b[5C");
    assert_eq!(codes::move_right!(0), "");
    assert_eq!(codes::move_left!(5), "\x1b[5D");
    assert_eq!(codes::move_left!(0), "");
    assert_eq!(codes::insert_lines!(5), "\x1b[5L");
    assert_eq!(codes::insert_lines!(0), "");
    assert_eq!(codes::delete_lines!(5), "\x1b[5M");
    assert_eq!(codes::delete_lines!(0), "");
    assert_eq!(codes::insert_chars!(5), "\x1b[5@");
    assert_eq!(codes::insert_chars!(0), "");
    assert_eq!(codes::delete_chars!(5), "\x1b[5P");
    assert_eq!(codes::delete_chars!(0), "");
    assert_eq!(codes::insert_columns!(5), "\x1b[5'}");
    assert_eq!(codes::insert_columns!(six), "\x1b[6'}");
    assert_eq!(codes::insert_columns!(0), "");
    assert_eq!(codes::delete_columns!(5), "\x1b[5'~");
    assert_eq!(codes::delete_columns!(0), "");
    assert_eq!(codes::set_down!(5), "\x1b[5E");
    assert_eq!(codes::set_down!(0), "");
    assert_eq!(codes::set_up!(5), "\x1b[5F");
    assert_eq!(codes::set_up!(0), "");

    assert_eq!(codes::column!(5), "\x1b[5G");
    assert_eq!(codes::column!(six), "\x1b[6G");

    assert_eq!(codes::fg256!(56), "\x1b[38;5;56m");
    assert_eq!(codes::fg256!(six), "\x1b[38;5;6m");
    assert_eq!(codes::bg256!(56), "\x1b[48;5;56m");
    assert_eq!(codes::underline256!(56), "\x1b[58;5;56m");
    assert_eq!(codes::fg!(12, 34, 56), "\x1b[38;2;12;34;56m");
    assert_eq!(codes::fg!(12, 34, six), "\x1b[38;2;12;34;6m");
    assert_eq!(codes::bg!(12, 34, 56), "\x1b[48;2;12;34;56m");
    assert_eq!(codes::underline_rgb!(12, 34, 56), "\x1b[58;2;12;34;56m");

    assert_eq!(codes::scroll_region!(12, 34), "\x1b[12;34r");
    assert_eq!(codes::scroll_region!(12, six), "\x1b[12;6r");

    assert_eq!(codes::set_cursor(CursorStyle::Block(None)), "\x1b[1 q");
    assert_eq!(codes::set_cursor(CursorStyle::Underline(true)), "\x1b[3 q");
    assert_eq!(codes::set_cursor(CursorStyle::Underline(false)), "\x1b[4 q");
    assert_eq!(codes::set_cursor(CursorStyle::Bar(true)), "\x1b[5 q");
    assert_eq!(codes::set_cursor(CursorStyle::Bar(false)), "\x1b[6 q");

    assert_eq!(codes::request_color_code!(11), "\x1b]4;11;?\x1b\\");
    assert_eq!(codes::request_color_code!(six), "\x1b]4;6;?\x1b\\");
    assert_eq!(codes::reset_color_code!(11), "\x1b]104;11\x1b\\");
    assert_eq!(codes::reset_color_code!(six), "\x1b]104;6\x1b\\");
}

#[test]
fn functions() {
    assert_eq!(
        codes::set_cursor(CursorStyle::Block(Some(true))),
        "\x1b[0 q"
    );
    assert_eq!(
        codes::set_cursor(CursorStyle::Block(Some(false))),
        "\x1b[2 q"
    );

    assert_eq!(
        codes::define_color_code(11, (0x12, 0x34, 0x56)),
        "\x1b]4;11;rgb:12/34/56\x1b\\"
    );
    assert_eq!(
        codes::define_color_code(11, (0x11, 0x33, 0x55)),
        "\x1b]4;11;rgb:1/3/5\x1b\\"
    );
    assert_eq!(
        codes::define_color_code(11, Rgb::<u16>::new(0x1212, 0x3434, 0x5656)),
        "\x1b]4;11;rgb:12/34/56\x1b\\"
    );
    assert_eq!(
        codes::define_color_code(11, Rgb::<u16>::new(0x1111, 0x3333, 0x5555)),
        "\x1b]4;11;rgb:1/3/5\x1b\\"
    );
    assert_eq!(
        codes::define_color_code(11, Rgb::<u16>::new(0x1234, 0x5678, 0x9abc)),
        "\x1b]4;11;rgb:1234/5678/9abc\x1b\\"
    );
    assert_eq!(
        codes::define_color_code(11, Rgb::<u16>::new(0x1231, 0x5675, 0x9ab9)),
        "\x1b]4;11;rgb:123/567/9ab\x1b\\"
    );
    assert_eq!(
        codes::set_default_fg_color((0x12, 0x34, 0x56)),
        "\x1b]10;rgb:12/34/56\x1b\\"
    );
    assert_eq!(
        codes::set_default_bg_color((0x12, 0x34, 0x56)),
        "\x1b]11;rgb:12/34/56\x1b\\"
    );
    assert_eq!(
        codes::set_cursor_color((0x12, 0x34, 0x56)),
        "\x1b]12;rgb:12/34/56\x1b\\"
    );

    assert_eq!(codes::request_selection([]), codes::REQUEST_SELECTION);
    assert_eq!(
        codes::request_selection([Selection::Select, Selection::Cut0]),
        "\x1b]52;s0;?\x1b\\"
    );
    assert_eq!(
        codes::set_selection([], b"hello"),
        "\x1b]52;;aGVsbG8=\x1b\\"
    );
    assert_eq!(
        codes::set_selection([Selection::Select, Selection::Cut0], b"hello"),
        "\x1b]52;s0;aGVsbG8=\x1b\\"
    );
}
