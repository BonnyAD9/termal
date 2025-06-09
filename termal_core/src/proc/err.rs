use std::{borrow::Cow, fmt::Display};

use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
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
    pub(crate) fn spanned<S>(span: Span, msg: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self {
            msg: msg.into(),
            span,
        }
    }

    pub(crate) fn msg<S>(msg: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::spanned(Span::call_site(), msg)
    }

    pub(crate) fn set_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    /// Converts the error to a [`TokenStream`] that will produce the error at
    /// compile time.
    pub fn to_stream(self) -> TokenStream {
        self.into()
    }

    /// Gets the [`Span`] of the error message.
    /// 
    /// This is the span that will be asociated with the compile time error.
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