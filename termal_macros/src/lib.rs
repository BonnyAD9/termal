use ansi_codes as codes;
use std::iter::Peekable;

use litrs::StringLit;
use proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream,
    TokenTree,
};

/// Creates formatted and colorized string
#[proc_macro]
pub fn colorize(item: TokenStream) -> TokenStream {
    let mut i = item.into_iter();

    let pat = get_first_string_iteral(&mut i);

    let s = parse_template(pat.value());

    // the arguments to the macro
    let mut rargs = TokenStream::new();
    rargs.extend([TokenTree::Literal(Literal::string(&s))]);
    rargs.extend(i);

    // invoking the macro
    let mut res = TokenStream::new();
    res.extend(
        [
            TokenTree::Ident(Ident::new("format", Span::call_site())),
            TokenTree::Punct(Punct::new('!', Spacing::Alone)),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, rargs)),
        ]
        .into_iter(),
    );

    res
}

fn get_first_string_iteral(
    i: &mut impl Iterator<Item = TokenTree>,
) -> StringLit<String> {
    let first = i.next();
    if first.is_none() {
        panic!("This macro must have at least one argument");
    }
    let first = first.unwrap();

    let arg = match first {
        TokenTree::Literal(l) => StringLit::try_from(l),
        TokenTree::Group(g) => {
            return get_first_string_iteral(&mut g.stream().into_iter())
        }
        _ => panic!("The first argument must be string literal"),
    };

    match arg {
        Ok(l) => l,
        Err(_) => panic!("The first argument must be string literal"),
    }
}

fn parse_template(s: &str) -> String {
    let mut i = s.chars().peekable();
    let mut res = String::new();

    while let Some(c) = i.next() {
        match c {
            '{' => match i.next() {
                Some('\'') => parse_block(&mut res, &mut i),
                Some(c) => {
                    res.push('{');
                    res.push(c);
                }
                None => res.push('{'),
            },
            _ => res.push(c),
        }
    }

    res
}

fn parse_block<I>(res: &mut String, i: &mut Peekable<I>)
where
    I: Iterator<Item = char>,
{
    while let Some(c) = i.peek() {
        match c {
            c if c.is_ascii_alphabetic() || *c == '_' => {
                parse_variable(res, i)
            }
            '}' => {
                i.next();
                return;
            }
            '#' => parse_color(res, i),
            ',' | ' ' => _ = i.next(),
            _ => {
                panic!("Invalid color format, didn't expect character '{}'", c)
            }
        }
    }
}

fn parse_variable<I>(res: &mut String, i: &mut Peekable<I>)
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
            ',' | '}' | ' ' => break,
            _ => {
                panic!("Invalid color format, didn't expect character '{}'", c)
            }
        }
    }

    let var = match s.to_lowercase().as_str() {
        "reset" | "_" => codes::RESET,

        "bold" => codes::BOLD,
        "faint" | "f" => codes::FAINT,
        "italic" | "i" => codes::ITALIC,
        "underline" | "un" => codes::UNDERLINE,
        "blinking" | "blink" => codes::BLINKING,
        "inverse" => codes::INVERSE,
        "invisible" | "invis" => codes::INVISIBLE,
        "striketrough" | "strike" => codes::STRIKETROUGH,
        "double_underline" | "dunderline" | "dun" => codes::DOUBLE_UNDERLINE,

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
        _ => panic!("Unsupported color format variable {}", s),
    };

    res.push_str(var);
}

fn parse_color<I>(res: &mut String, i: &mut Peekable<I>)
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
            ',' | '}' | ' ' | '_' => break,
            _ => {
                panic!("Invalid hex color, didn't expect character '{}'", c)
            }
        }
    }

    // get the hex color
    let (r, g, b) = match s.len() {
        1 => {
            let mut c = u32::from_str_radix(&s, 16).unwrap();
            c |= c >> 4;
            (c, c, c)
        }
        2 => {
            let c = u32::from_str_radix(&s, 16).unwrap();
            (c, c, c)
        }
        3 => {
            let c = u32::from_str_radix(&s, 16).unwrap();
            (
                (c & 0xF00) >> 4 | (c & 0xF00) >> 8,
                (c & 0x0F0) | (c & 0x0F0) >> 4,
                (c & 0x00F) << 4 | (c & 0x00F),
            )
        }
        6 => {
            let c = u32::from_str_radix(&s, 16).unwrap();
            (c & 0xFF0000 >> 16, c & 0x00FF00 >> 8, c & 0x0000FF)
        }
        _ => panic!("Invalid hex color length, must be 1, 2, 3 or 6"),
    };

    match i.peek() {
        Some('_') => {
            i.next();
            res.push_str(codes::bg!(r, g, b).as_str());
        }
        _ => res.push_str(codes::fg!(r, g, b).as_str()),
    }
}
