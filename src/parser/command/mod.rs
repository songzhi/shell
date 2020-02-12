use crate::error::{ProximateShellError, ShellError};
use crate::parser::hir::NamedArguments;
use crate::parser::span::Span;
use crate::parser::token::{SpannedToken, Token};
use crate::signature::{NamedType, Signature};

pub mod classified;

type OptionalHeadTail = (Option<Vec<SpannedToken>>, Option<NamedArguments>);

pub fn parse_command_tail(
    config: &Signature,
    tail: &mut impl Iterator<Item = SpannedToken>,
    command_span: Span,
    source: &str,
) -> Result<Option<OptionalHeadTail>, ShellError> {
    let mut named = NamedArguments::new();
    let mut positional: Vec<SpannedToken> = vec![];
    let mut rest_signature = config.clone();
    while let Some(spanned) = tail.next() {
        match spanned.item {
            Token::String(_) | Token::Bare | Token::ExternalWord | Token::GlobPattern => {
                if !rest_signature.positional.is_empty() {
                    positional.push(spanned);
                    rest_signature.shift_positional();
                } else if config.rest_positional.is_some() {
                    positional.push(spanned);
                }
            }
            Token::Flag(flag) => {
                if let Some((kind, _)) = config.named.get(flag.slice(source)) {
                    let kind: &NamedType = kind; // just type annotation
                    match kind {
                        NamedType::Switch => {
                            rest_signature.remove_named(flag.slice(source));
                            named.insert_switch(flag.slice(source), Some(flag));
                        }
                        NamedType::Mandatory(_) => {
                            if let Some(next_token) = tail.next() {
                                rest_signature.remove_named(flag.slice(source));
                                named.insert_mandatory(flag.slice(source), next_token);
                            } else {
                                break;
                            }
                        }
                        NamedType::Optional(_) => {
                            let next_token = tail.next();
                            rest_signature.remove_named(flag.slice(source));
                            named.insert_optional(flag.slice(source), next_token);
                        }
                    }
                }
            }
            Token::Whitespace | Token::Separator | Token::Pipeline(_) => {}
        }
    }
    let mut err: Option<(Span, Option<String>)> = None;
    if let Some((positional_type, _)) = rest_signature
        .positional
        .iter()
        .find(|p| p.0.is_mandatory())
    {
        err = Some((
            command_span,
            Some(format!(
                "{} needs positional parameter: {}",
                config.name,
                positional_type.name()
            )),
        ));
    }
    if err.is_none() {
        if let Some((name, _)) = rest_signature
            .named
            .iter()
            .find(|(_, kind)| kind.0.is_mandatory())
        {
            err = Some((
                command_span,
                Some(format!("{} needs named parameter: {}", config.name, name)),
            ));
        }
    }
    if let Some((span, reason)) = err {
        Err(ProximateShellError::ParseError(span, reason).start())
    } else {
        let positional = if positional.is_empty() {
            None
        } else {
            Some(positional)
        };
        let named = if named.named.is_empty() {
            None
        } else {
            Some(named)
        };
        if positional.is_none() && positional.is_none() {
            Ok(None)
        } else {
            Ok(Some((positional, named)))
        }
    }
}
