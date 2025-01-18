//! Procedural macros implemented with `proc_macro2`.

use crate::{
    codes::{self as codes},
    move_to,
};
use std::{borrow::Cow, fmt::Display, iter::Peekable};

use litrs::StringLit;
use proc_macro2::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream,
    TokenTree,
};
use thiserror::Error;

/// Error of termal procedural macro. Can be converted to a [`TokenStream`]
/// that produces the error message or the message can be printed with the
/// [`Display`] implementation.
#[derive(Error, Debug)]
pub struct ProcError {
    msg: Cow<'static, str>,
    span: Span,
}

impl Display for ProcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg.as_ref())
    }
}

impl From<ProcError> for TokenStream {
    fn from(value: ProcError) -> Self {
        error_at(value.span, value.msg)
    }
}

impl ProcError {
    fn spanned<S>(span: Span, msg: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self {
            msg: msg.into(),
            span,
        }
    }

    fn msg<S>(msg: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::spanned(Span::call_site(), msg)
    }

    fn set_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    /// Converts self to a [`TokenStream`]
    pub fn to_stream(self) -> TokenStream {
        self.into()
    }

    /// Gets the [`Span`] of the error message
    pub fn span(&self) -> Span {
        self.span
    }
}

fn spanned(mut tree: TokenTree, span: Span) -> TokenTree {
    tree.set_span(span);
    tree
}

fn error_at<S>(span: Span, msg: S) -> TokenStream
where
    S: AsRef<str>,
{
    [
        TokenTree::Ident(Ident::new("compile_error", span)),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        spanned(
            TokenTree::Group(Group::new(
                Delimiter::Parenthesis,
                [TokenTree::Literal(Literal::string(msg.as_ref()))]
                    .into_iter()
                    .collect(),
            )),
            span,
        ),
    ]
    .into_iter()
    .collect()
}

/// Result type for termal procedural macros
pub type ProcResult<T> = Result<T, ProcError>;

/// Creates formatted and colorized string. Expands to call to a [`format!`]
/// macro. Doesn't panic, errors are signified with the result.
pub fn colorize(item: TokenStream) -> ProcResult<TokenStream> {
    let mut i = item.into_iter();

    let (pat, span) = get_first_string_iteral(&mut i)?;

    let s = parse_template(pat.value()).map_err(|e| e.set_span(span))?;
    let mut s = Literal::string(&s);
    s.set_span(span);

    // the arguments to the macro
    let mut rargs = TokenStream::new();
    rargs.extend([TokenTree::Literal(s)]);
    rargs.extend(i);

    // invoking the macro
    let mut res = TokenStream::new();
    res.extend([
        TokenTree::Ident(Ident::new("format", Span::call_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, rargs)),
    ]);

    Ok(res)
}

/// Removes terminal commands from the string. Expands to call to a [`format!`]
/// macro. Doesn't panic, errors are signified with the result.
pub fn uncolor(item: TokenStream) -> ProcResult<TokenStream> {
    let mut i = item.into_iter();

    let (pat, span) = get_first_string_iteral(&mut i)?;

    let s = skip_colors(pat.value()).map_err(|e| e.set_span(span))?;
    let mut s = Literal::string(&s);
    s.set_span(span);

    // the arguments to the macro
    let mut rargs = TokenStream::new();
    rargs.extend([TokenTree::Literal(s)]);
    rargs.extend(i);

    // invoking the macro
    let mut res = TokenStream::new();
    res.extend([
        TokenTree::Ident(Ident::new("format", Span::call_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, rargs)),
    ]);

    Ok(res)
}

fn get_first_string_iteral(
    i: &mut impl Iterator<Item = TokenTree>,
) -> ProcResult<(StringLit<String>, Span)> {
    let first = if let Some(first) = i.next() {
        first
    } else {
        return Err(ProcError::msg(
            "This macro must have at least one argument",
        ));
    };

    let (arg, span) = match first {
        TokenTree::Literal(l) => {
            let span = l.span();
            (StringLit::try_from(l), span)
        }
        TokenTree::Group(g) => {
            return get_first_string_iteral(&mut g.stream().into_iter())
        }
        _ => {
            return Err(ProcError::spanned(
                first.span(),
                "The first argument must be string literal",
            ))
        }
    };

    match arg {
        Ok(l) => Ok((l, span)),
        Err(_) => Err(ProcError::spanned(
            span,
            "The first argument must be string literal",
        )),
    }
}

fn skip_colors(s: &str) -> ProcResult<String> {
    let mut i = s.chars().peekable();
    let mut res = String::new();

    while let Some(c) = i.next() {
        match c {
            '{' => match i.next() {
                Some('\'') => skip_block(&mut i)?,
                Some(c) => {
                    res.push('{');
                    res.push(c);
                }
                None => res.push('{'),
            },
            _ => res.push(c),
        }
    }

    Ok(res)
}

fn skip_block<I>(i: &mut Peekable<I>) -> ProcResult<()>
where
    I: Iterator<Item = char>,
{
    while let Some(c) = i.peek() {
        match c {
            '}' => {
                i.next();
                return Ok(());
            }
            _ => _ = i.next(),
        }
    }

    Err(ProcError::msg("Missing '}}' at the end of color pattern"))
}

fn parse_template(s: &str) -> ProcResult<String> {
    let mut i = s.chars().peekable();
    let mut res = String::new();

    while let Some(c) = i.next() {
        match c {
            '{' => match i.next() {
                Some('\'') => parse_block(&mut res, &mut i)?,
                Some(c) => {
                    res.push('{');
                    res.push(c);
                }
                None => res.push('{'),
            },
            _ => res.push(c),
        }
    }

    Ok(res)
}

fn parse_block<I>(res: &mut String, i: &mut Peekable<I>) -> ProcResult<()>
where
    I: Iterator<Item = char>,
{
    while let Some(c) = i.peek() {
        match c {
            c if c.is_ascii_alphabetic() || *c == '_' => {
                parse_variable(res, i)?
            }
            '}' => {
                i.next();
                return Ok(());
            }
            '#' => parse_color(res, i)?,
            ' ' => _ = i.next(),
            _ => {
                return Err(ProcError::msg(format!(
                    "Invalid color format, didn't expect character '{}'",
                    c
                )));
            }
        }
    }

    Err(ProcError::msg("Missing '}}' at the end of color pattern"))
}

fn parse_variable<I>(res: &mut String, i: &mut Peekable<I>) -> ProcResult<()>
where
    I: Iterator<Item = char>,
{
    let mut s = String::new();

    while let Some(c) = i.peek() {
        match c {
            c if c.is_ascii_alphabetic() || *c == '_' => {
                s.push(*c);
                i.next();
            }
            '}' | ' ' => break,
            c if c.is_ascii_digit() || *c == ',' => break,
            _ => {
                return Err(ProcError::msg(format!(
                    "Invalid color format, didn't expect character '{}'",
                    c
                )));
            }
        }
    }

    /// macro, default, owner
    macro_rules! m_arm {
        ($m:ident, $d:literal, $o:ident) => {{
            $o = codes::$m!(maybe_read_num(i).unwrap_or($d));
            &$o
        }};
    }

    let owner;

    let var = match s.to_lowercase().as_str() {
        "bell" => "\x07",
        "backspace" => "\x08",
        "htab" | "tab" => "\t",
        "move_down_scrl" | "mds" => "\n",
        "newline" | "nl" => "\n\r",
        "vtab" => "\x0b",
        "carriage_return" | "cr" => "\r",
        "delete" | "del" => "\x7f",

        "move_to" | "mt" => {
            let x = maybe_read_num(i);
            if matches!(i.peek(), Some(',')) && x.is_some() {
                i.next();
            } else if x.is_some() {
                return Err(ProcError::msg(format!(
                    "'{}', takes two arguments",
                    s
                )));
            }
            let y = maybe_read_num(i);
            if x.is_none() && y.is_none() {
                "\x1b[H"
            } else {
                owner = move_to!(x.unwrap_or_default(), y.unwrap_or_default());
                &owner
            }
        }
        "move_up" | "mu" => m_arm!(move_up, 1, owner),
        "move_down" | "md" => m_arm!(move_down, 1, owner),
        "move_right" | "mr" => m_arm!(move_right, 1, owner),
        "move_left" | "ml" => m_arm!(move_left, 1, owner),
        "set_down" | "sd" => m_arm!(set_down, 1, owner),
        "set_up" | "su" => m_arm!(set_up, 1, owner),
        "move_to_column" | "mc" => m_arm!(column, 0, owner),

        "move_up_scrl" | "mus" => codes::UP_SCRL,
        "save_cur" | "save" | "s" => codes::CUR_SAVE,
        "load_cur" | "load" | "l" => codes::CUR_LOAD,

        "erase_to_end" | "e_" => codes::ERASE_TO_END,
        "erase_from_start" | "_e" => codes::ERASE_FROM_START,
        "erase_screen" | "_e_" => codes::ERASE_SCREEN,
        "erase_all" | "e" => codes::ERASE_ALL,
        "erase_ln_end" | "el_" => codes::ERASE_TO_LN_END,
        "erase_ln_start" | "_el" => codes::ERASE_FROM_LN_START,
        "erase_line" | "erase_ln" | "_el_" | "el" => codes::ERASE_LINE,

        "reset" | "_" => codes::RESET,

        "bold" => codes::BOLD,
        "faint" | "f" => codes::FAINT,
        "italic" | "i" => codes::ITALIC,
        "underline" | "u" => codes::UNDERLINE,
        "blinking" | "blink" => codes::BLINKING,
        "inverse" => codes::INVERSE,
        "invisible" | "invis" => codes::INVISIBLE,
        "striketrough" | "strike" => codes::STRIKETROUGH,
        "double_underline" | "dunderline" | "dun" => codes::DOUBLE_UNDERLINE,
        "overline" | "ol" => codes::OVERLINE,

        "_bold" => codes::RESET_BOLD,
        "_italic" | "_i" => codes::RESET_ITALIC,
        "_underline" | "_u" => codes::RESET_UNDERLINE,
        "_blinking" | "_blink" => codes::RESET_BLINKING,
        "_inverse" => codes::RESET_INVERSE,
        "_invisible" | "_invis" => codes::RESET_INVISIBLE,
        "_striketrough" | "_strike" => codes::RESET_STRIKETROUGH,

        "black_fg" | "black" | "bl" => codes::BLACK_FG,
        "white_fg" | "white" | "w" => codes::WHITE_FG,
        "gray_fg" | "gray" | "gr" => codes::GRAY_FG,
        "bright_gray_fg" | "bgray" | "bgr" => codes::GRAY_BRIGHT_FG,

        "red_fg" | "red" | "r" => codes::RED_FG,
        "green_fg" | "green" | "g" => codes::GREEN_FG,
        "yellow_fg" | "yellow" | "y" => codes::YELLOW_FG,
        "blue_fg" | "blue" | "b" => codes::BLUE_FG,
        "magenta_fg" | "magenta" | "m" => codes::MAGENTA_FG,
        "cyan_fg" | "cyan" | "c" => codes::CYAN_FG,

        "dark_red_fg" | "dred" | "dr" => codes::RED_DARK_FG,
        "dark_green_fg" | "dgreen" | "dg" => codes::GREEN_DARK_FG,
        "dark_yellow_fg" | "dyellow" | "dy" => codes::YELLOW_DARK_FG,
        "dark_blue_fg" | "dblue" | "db" => codes::BLUE_DARK_FG,
        "dark_magenta_fg" | "dmagenta" | "dm" => codes::MAGENTA_DARK_FG,
        "dark_cyan_fg" | "dcyan" | "dc" => codes::CYAN_DARK_FG,

        "_fg" => codes::RESET_FG,

        "black_bg" | "blackb" | "blb" => codes::BLACK_BG,
        "white_bg" | "whiteb" | "wb" => codes::WHITE_BG,
        "gray_bg" | "grayb" | "grb" => codes::GRAY_BG,
        "bright_gray_bg" | "bgrayb" | "bgrb" => codes::GRAY_BRIGHT_BG,

        "red_bg" | "redb" | "rb" => codes::RED_BG,
        "green_bg" | "greenb" | "gb" => codes::GREEN_BG,
        "yellow_bg" | "yellowb" | "yb" => codes::YELLOW_BG,
        "blue_bg" | "blueb" | "bb" => codes::BLUE_BG,
        "magenta_bg" | "magentab" | "mb" => codes::MAGENTA_BG,
        "cyan_bg" | "cyanb" | "cb" => codes::CYAN_BG,

        "dark_red_bg" | "dredb" | "drb" => codes::RED_DARK_BG,
        "dark_green_bg" | "dgreenb" | "dgb" => codes::GREEN_DARK_BG,
        "dark_yellow_bg" | "dyellowb" | "dyb" => codes::YELLOW_DARK_BG,
        "dark_blue_bg" | "dblueb" | "dbb" => codes::BLUE_DARK_BG,
        "dark_magenta_bg" | "dmagentab" | "dmb" => codes::MAGENTA_DARK_BG,
        "dark_cyan_bg" | "dcyanb" | "dcb" => codes::CYAN_DARK_BG,

        "_bg" => codes::RESET_BG,

        "fg" => {
            let c = match maybe_read_num(i) {
                Some(c) if (0..256).contains(&c) => c,
                _ => {
                    return Err(ProcError::msg(format!(
                    "The '{}' in color format expects value in range 0..256",
                    s,
                )))
                }
            };
            owner = codes::fg256!(c);
            &owner
        }
        "bg" => {
            let c = match maybe_read_num(i) {
                Some(c) if (0..256).contains(&c) => c,
                _ => {
                    return Err(ProcError::msg(format!(
                    "The '{}' in color format expects value in range 0..256",
                    s,
                )))
                }
            };
            owner = codes::bg256!(c);
            &owner
        }

        "line_wrap" | "wrap" => codes::ENABLE_LINE_WRAP,
        "_line_wrap" | "_wrap" => codes::DISABLE_LINE_WRAP,

        "hide_cursor" | "nocur" => codes::HIDE_CURSOR,
        "show_cursor" | "_nocur" => codes::SHOW_CURSOR,
        "save_screen" | "sscr" => codes::SAVE_SCREEN,
        "load_screen" | "lscr" => codes::LOAD_SCREEN,
        "alt_buf" | "abuf" => codes::ENABLE_ALTERNATIVE_BUFFER,
        "_alt_buf" | "_abuf" => codes::DISABLE_ALTERNATIVE_BUFFER,

        "clear" | "cls" => {
            owner = format!("{}\x1b[H", codes::ERASE_ALL);
            &owner
        }
        _ => {
            return Err(ProcError::msg(format!(
                "Unknown color format variable {}",
                s
            )))
        }
    };

    match i.peek() {
        Some(' ' | '}') => {}
        Some(c) => {
            return Err(ProcError::msg(format!(
                "Invalid character '{}', expected ' ' or '}}'",
                c
            )))
        }
        None => {
            return Err(ProcError::msg(
                "Unexpected end, expected ' ' or '}}'".to_owned(),
            ))
        }
    }

    res.push_str(var);

    Ok(())
}

fn parse_color<I>(res: &mut String, i: &mut Peekable<I>) -> ProcResult<()>
where
    I: Iterator<Item = char>,
{
    i.next();
    let mut s = String::new();

    while let Some(c) = i.peek() {
        match c {
            c if c.is_ascii_hexdigit() => {
                s.push(*c);
                i.next();
            }
            '}' | ' ' | '_' => break,
            _ => {
                return Err(ProcError::msg(format!(
                    "Invalid hex color, didn't expect character '{}'",
                    c
                )))
            }
        }
    }

    let c = if let Ok(c) = u32::from_str_radix(&s, 16) {
        c
    } else {
        return Err(ProcError::msg("Invalid hex color"));
    };

    // get the hex color
    let (r, g, b) = match s.len() {
        1 => {
            let c = c | (c << 4);
            (c, c, c)
        }
        2 => (c, c, c),
        3 => (
            (c & 0xF00) >> 4 | (c & 0xF00) >> 8,
            (c & 0x0F0) | (c & 0x0F0) >> 4,
            (c & 0x00F) << 4 | (c & 0x00F),
        ),
        6 => ((c & 0xFF0000) >> 16, (c & 0x00FF00) >> 8, c & 0x0000FF),
        _ => {
            return Err(ProcError::msg(
                "Invalid hex color length, must be 1, 2, 3 or 6".to_owned(),
            ))
        }
    };

    match i.peek() {
        Some('_') => {
            i.next();
            res.push_str(codes::bg!(r, g, b).as_str());
            Ok(())
        }
        Some(' ' | '}') => {
            res.push_str(codes::fg!(r, g, b).as_str());
            Ok(())
        }
        Some(c) => Err(ProcError::msg(format!(
            "Invalid character, didn't expect '{}'",
            c
        ))),
        None => Err(ProcError::msg(
            "color format not ended with '}}'".to_owned(),
        )),
    }
}

fn maybe_read_num<I>(i: &mut Peekable<I>) -> Option<i32>
where
    I: Iterator<Item = char>,
{
    let mut s = String::new();
    read_while(&mut s, i, |c| c.is_ascii_digit());
    s.parse().ok()
}

fn read_while<I, F>(res: &mut String, i: &mut Peekable<I>, f: F)
where
    I: Iterator<Item = char>,
    F: Fn(char) -> bool,
{
    while let Some(c) = i.peek() {
        if f(*c) {
            res.push(*c);
            i.next();
        } else {
            break;
        }
    }
}
