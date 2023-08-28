use std::iter::Peekable;
use ansi_codes as codes;

use litrs::StringLit;
use proc_macro::{
    Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree, Group, Delimiter,
};

/// Creates formatted and colorized string
#[proc_macro]
pub fn colorize(item: TokenStream) -> TokenStream {
    let mut i = item.into_iter();

    let pat = get_first_string_iteral(&mut i);

    let s = parse_template(pat.value());

    // the arguments to the macro
    let mut rargs = TokenStream::new();
    rargs.extend([
        TokenTree::Literal(Literal::string(&s)),
    ]);
    rargs.extend(i);

    // invoking the macro
    let mut res = TokenStream::new();
    res.extend(
        [
            TokenTree::Ident(Ident::new("format", Span::call_site())),
            TokenTree::Punct(Punct::new('!', Spacing::Alone)),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, rargs))
        ]
        .into_iter(),
    );

    res
}

fn get_first_string_iteral(i: &mut impl Iterator<Item = TokenTree>) -> StringLit<String> {
    let first = i.next();
    if first.is_none() {
        panic!("This macro must have at least one argument");
    }
    let first = first.unwrap();

    let arg = match first {
        TokenTree::Literal(l) => StringLit::try_from(l),
        TokenTree::Group(g) => return get_first_string_iteral(&mut g.stream().into_iter()),
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
            c if c.is_ascii_alphabetic() || *c == '_' => parse_variable(res, i),
            '}' => {
                i.next();
                return;
            }
            '#' => parse_color(res, i),
            ',' => _ = i.next(),
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
            ',' | '}' => break,
            _ => {
                panic!("Invalid color format, didn't expect character '{}'", c)
            }
        }
    }

    let var = match s.to_lowercase().as_str() {
        "reset" => codes::RESET,
        "black_fg" | "blackfg" | "black" => codes::BLACK_FG,
        "white_fg" | "whitefg" | "white" => codes::WHITE_FG,
        "gray_fg" | "grayfg" | "gray" => codes::GRAY_FG,
        "gray_bright_fg" | "graybrightfg" | "bgray" => codes::GRAY_BRIGHT_FG,
        "red_fg" | "redfg" | "red" => codes::RED_FG,
        "green_fg" | "greenfg" | "green" => codes::RED_FG,
        "yellow_fg" | "yellowfg" | "yellow" => codes::RED_FG,
        "blue_fg" | "bluefg" | "blue" => codes::RED_FG,
        "magenta_fg" | "magentafg" | "magenta" => codes::RED_FG,
        "cyan_fg" | "cyanfg" | "cyan" => codes::RED_FG,
        _ => panic!("Unsupported color format variable {}", s),
    };

    res.push_str(var);
}

fn parse_color<I>(res: &mut String, i: &mut Peekable<I>)
where
    I: Iterator<Item = char>,
{
    todo!()
}
