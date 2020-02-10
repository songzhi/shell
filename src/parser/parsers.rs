use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, none_of, space1};
use nom::IResult;
use nom::multi::{many0, many1};
use nom_tracable::{HasTracableInfo, tracable_parser, TracableInfo};

use crate::parser::span::{Span, SpannedItem};
use crate::parser::token::{SpannedToken, Token};

use super::tracable::{nom_input, NomSpan, TracableContext};

#[tracable_parser]
pub fn dq_string(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let start = input.offset;
    let (input, _) = char('"')(input)?;
    let start1 = input.offset;
    let (input, _) = many0(none_of("\""))(input)?;
    let end1 = input.offset;
    let (input, _) = char('"')(input)?;
    let end = input.offset;

    Ok((
        input,
        Token::String(Span::new(start1, end1)).spanned(Span::new(start, end))
    ))
}

#[tracable_parser]
pub fn sq_string(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let start = input.offset;
    let (input, _) = char('\'')(input)?;
    let start1 = input.offset;
    let (input, _) = many0(none_of("\'"))(input)?;
    let end1 = input.offset;
    let (input, _) = char('\'')(input)?;
    let end = input.offset;

    Ok((
        input,
        Token::String(Span::new(start1, end1)).spanned(Span::new(start, end))
    ))
}

#[tracable_parser]
pub fn string(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    alt((sq_string, dq_string))(input)
}

#[tracable_parser]
pub fn separator(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let left = input.offset;
    let (input, ws1) = alt((tag(";"), tag("\n")))(input)?;
    let right = input.offset;

    Ok((input, Token::Separator.spanned(Span::new(left, right))))
}

#[tracable_parser]
pub fn whitespace(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let left = input.offset;
    let (input, _) = space1(input)?;
    let right = input.offset;

    Ok((input, Token::Whitespace.spanned(Span::new(left, right))))
}

#[tracable_parser]
pub fn any_space(input: NomSpan) -> IResult<NomSpan, Vec<SpannedToken>> {
    let (input, tokens) = many1(alt((whitespace, separator)))(input)?;

    Ok((input, tokens))
}
