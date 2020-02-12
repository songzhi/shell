use serde::{Deserialize, Serialize};

use crate::parser::span::Span;
use crate::parser::tracable::TracableContext;

/// A `ShellError` is a proximate error and a possible cause, which could have its own cause,
/// creating a cause chain.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Serialize, Deserialize, Hash)]
pub struct ShellError {
    error: ProximateShellError,
    cause: Option<Box<ShellError>>,
}

impl ShellError {
    pub fn parse_error(
        error: nom::Err<(
            nom_locate::LocatedSpanEx<&str, TracableContext>,
            nom::error::ErrorKind,
        )>,
    ) -> ShellError {
        let reason = Some(String::from("parse error"));
        match error {
            nom::Err::Incomplete(s) => {
                ProximateShellError::ParseError(Span::unknown(), reason).start()
            }
            nom::Err::Failure(span) | nom::Err::Error(span) => {
                ProximateShellError::ParseError(Span::from(span.0), reason).start()
            }
        }
    }
    pub fn runtime_error(reason: impl Into<String>) -> ShellError {
        ProximateShellError::RuntimeError(reason.into()).start()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Serialize, Deserialize, Hash)]
pub enum ProximateShellError {
    ParseError(Span, Option<String>),
    RuntimeError(String),
}

impl ProximateShellError {
    pub fn start(self) -> ShellError {
        ShellError {
            cause: None,
            error: self,
        }
    }
}
