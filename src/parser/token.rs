use std::ops::Deref;

use crate::parser::span::Spanned;

use super::span::{HasSpan, Span};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Token {
    String(Span),
    Bare,
    Flag(Span),
    Whitespace,
    Separator,
    GlobPattern,
    ExternalWord,
}

pub type SpannedToken = Spanned<Token>;

impl From<&SpannedToken> for Span {
    fn from(token: &SpannedToken) -> Span {
        token.span
    }
}

impl SpannedToken {}
