use termal::codes::{self, CursorStyle};

#[test]
fn test_macros() {
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
    assert_eq!(codes::fg!(12, 34, 56), "\x1b[38;2;12;34;56m");
    assert_eq!(codes::fg!(12, 34, six), "\x1b[38;2;12;34;6m");
    assert_eq!(codes::bg!(12, 34, 56), "\x1b[48;2;12;34;56m");

    assert_eq!(codes::scroll_region!(12, 34), "\x1b[12;34r");
    assert_eq!(codes::scroll_region!(12, six), "\x1b[12;6r");

    assert_eq!(
        codes::set_cursor(CursorStyle::Block(Some(true))),
        "\x1b[0 q"
    );
    assert_eq!(
        codes::set_cursor(CursorStyle::Block(Some(false))),
        "\x1b[2 q"
    );
    assert_eq!(codes::set_cursor(CursorStyle::Block(None)), "\x1b[1 q");
    assert_eq!(codes::set_cursor(CursorStyle::Underline(true)), "\x1b[3 q");
    assert_eq!(codes::set_cursor(CursorStyle::Underline(false)), "\x1b[4 q");
    assert_eq!(codes::set_cursor(CursorStyle::Bar(true)), "\x1b[5 q");
    assert_eq!(codes::set_cursor(CursorStyle::Bar(false)), "\x1b[6 q");
}
