use enumflags2::BitFlags;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{char, none_of, space1};
use nom::combinator::opt;
use nom::error::ParseError;
use nom::multi::{many0, many1};
use nom::{IResult, InputIter, Slice};
use nom_tracable::{tracable_parser, HasTracableInfo, TracableInfo};

use span::{Span, Spanned, SpannedItem};
use token::{SpannedToken, Token};
use tracable::{nom_input, NomSpan, TracableContext};

pub mod signature;
pub mod span;
pub mod syntax_shape;
pub mod token;
pub mod tracable;

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
        Token::String(Span::new(start1, end1)).spanned(Span::new(start, end)),
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
        Token::String(Span::new(start1, end1)).spanned(Span::new(start, end)),
    ))
}

#[tracable_parser]
pub fn string(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    alt((sq_string, dq_string))(input)
}

#[tracable_parser]
pub fn separator(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let left = input.offset;
    let (input, _) = alt((tag(";"), tag("\n")))(input)?;
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

fn word<'a, T, U, V>(
    start_predicate: impl Fn(NomSpan<'a>) -> IResult<NomSpan<'a>, U>,
    next_predicate: impl Fn(NomSpan<'a>) -> IResult<NomSpan<'a>, V> + Copy,
    into: impl Fn(Span) -> T,
) -> impl Fn(NomSpan<'a>) -> IResult<NomSpan<'a>, T> {
    move |input: NomSpan| {
        let start = input.offset;

        let (input, _) = start_predicate(input)?;
        let (input, _) = many0(next_predicate)(input)?;

        let next_char = &input.fragment.chars().nth(0);

        match next_char {
            Some('.') => {}
            Some(next_char)
                if is_external_word_char(*next_char) || is_glob_specific_char(*next_char) =>
            {
                return Err(nom::Err::Error(nom::error::make_error(
                    input,
                    nom::error::ErrorKind::TakeWhile1,
                )));
            }
            _ => {}
        }

        let end = input.offset;

        Ok((input, into(Span::new(start, end))))
    }
}

pub fn matches(cond: fn(char) -> bool) -> impl Fn(NomSpan) -> IResult<NomSpan, NomSpan> + Copy {
    move |input: NomSpan| match input.iter_elements().next() {
        Option::Some(c) if cond(c) => {
            let len_utf8 = c.len_utf8();
            Ok((input.slice(len_utf8..), input.slice(0..len_utf8)))
        }
        _ => Err(nom::Err::Error(nom::error::ParseError::from_error_kind(
            input,
            nom::error::ErrorKind::Many0,
        ))),
    }
}

#[tracable_parser]
pub fn pattern(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    word(start_pattern, matches(is_glob_char), |span| {
        Token::GlobPattern.spanned(span)
    })(input)
}

#[tracable_parser]
pub fn start_pattern(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    alt((take_while1(is_dot), matches(is_start_glob_char)))(input)
}

#[tracable_parser]
pub fn filename(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let start_pos = input.offset;

    let (mut input, mut saw_special) = match start_file_char(input) {
        Err(err) => return Err(err),
        Ok((input, special)) => (input, special),
    };

    loop {
        if saw_special.is_empty() {
            match continue_file_char(input) {
                Err(_) => {
                    return Ok((
                        input,
                        Token::Bare.spanned(Span::new(start_pos, input.offset)),
                    ));
                }
                Ok((next_input, special)) => {
                    saw_special |= special;
                    input = next_input;
                }
            }
        } else {
            let rest = after_sep_file(input);

            let (input, span, updated_special) = match rest {
                Err(_) => (input, (start_pos, input.offset), saw_special),
                Ok((input, new_special)) => {
                    (input, (start_pos, input.offset), saw_special | new_special)
                }
            };

            return if updated_special.contains(SawSpecial::Glob) {
                Ok((input, Token::GlobPattern.spanned(span)))
            } else {
                Ok((input, Token::Bare.spanned(span)))
            };
        }
    }
}

#[derive(BitFlags, Copy, Clone, Eq, PartialEq)]
enum SawSpecial {
    PathSeparator = 0b01,
    Glob = 0b10,
}

#[tracable_parser]
fn start_file_char(input: NomSpan) -> IResult<NomSpan, BitFlags<SawSpecial>> {
    let path_sep_result = special_file_char(input);

    if let Ok((input, special)) = path_sep_result {
        return Ok((input, special));
    }

    start_filename(input).map(|(input, output)| (input, BitFlags::empty()))
}

#[tracable_parser]
fn continue_file_char(input: NomSpan) -> IResult<NomSpan, BitFlags<SawSpecial>> {
    let path_sep_result = special_file_char(input);

    if let Ok((input, special)) = path_sep_result {
        return Ok((input, special));
    }

    matches(is_file_char)(input).map(|(input, _)| (input, BitFlags::empty()))
}

#[tracable_parser]
fn special_file_char(input: NomSpan) -> IResult<NomSpan, BitFlags<SawSpecial>> {
    if let Ok((input, _)) = matches(is_path_separator)(input) {
        return Ok((input, BitFlags::empty() | SawSpecial::PathSeparator));
    }

    let (input, _) = matches(is_glob_specific_char)(input)?;

    Ok((input, BitFlags::empty() | SawSpecial::Glob))
}

#[tracable_parser]
fn after_sep_file(input: NomSpan) -> IResult<NomSpan, BitFlags<SawSpecial>> {
    fn after_sep_char(c: char) -> bool {
        is_external_word_char(c) || is_file_char(c) || c == '.'
    }

    let start = input.offset;
    let original_input = input;
    let input = input;

    let (input, _) = take_while1(after_sep_char)(input)?;

    let slice = original_input.slice(0..input.offset - start);

    let saw_special = if slice.fragment.chars().any(is_glob_specific_char) {
        BitFlags::empty() | SawSpecial::Glob
    } else {
        BitFlags::empty()
    };

    Ok((input, saw_special))
}

pub fn start_filename(input: NomSpan) -> IResult<NomSpan, NomSpan> {
    alt((take_while1(is_dot), matches(is_start_file_char)))(input)
}

#[tracable_parser]
pub fn flag(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let start = input.offset;
    let (input, _) = tag("--")(input)?;
    let (input, bare) = filename(input)?;
    let end = input.offset;

    Ok((input, Token::Flag(bare.span).spanned(Span::new(start, end))))
}

#[tracable_parser]
pub fn external_word(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let start = input.offset;
    let (input, _) = take_while1(is_external_word_char)(input)?;
    let end = input.offset;

    Ok((input, Token::ExternalWord.spanned(Span::new(start, end))))
}

#[tracable_parser]
pub fn node(input: NomSpan) -> IResult<NomSpan, SpannedToken> {
    let (input, node) = alt((string, flag, filename, pattern, external_word))(input)?;

    Ok((input, node))
}

#[tracable_parser]
pub fn token_list(input: NomSpan) -> IResult<NomSpan, Spanned<Vec<SpannedToken>>> {
    let start = input.offset;
    let mut node_list = vec![];

    let mut next_input = input;
    let mut before_space_input: Option<NomSpan> = None;
    let mut final_space_tokens = 0;
    loop {
        let node_result = node(input);
        let (after_node_input, next_node) = match node_result {
            Err(_) => {
                if let Some(before_space_input) = before_space_input {
                    next_input = before_space_input;

                    for _ in 0..final_space_tokens {
                        node_list.pop();
                    }
                }

                break;
            }
            Ok((after_node_input, next_node)) => (after_node_input, next_node),
        };

        node_list.push(next_node);

        let maybe_space = any_space(after_node_input);

        let after_space_input = match maybe_space {
            Err(_) => {
                next_input = after_node_input;

                break;
            }
            Ok((after_space_input, space)) => {
                final_space_tokens = space.len();
                node_list.extend(space);
                before_space_input = Some(after_node_input);
                after_space_input
            }
        };

        next_input = after_space_input;
    }
    let end = next_input.offset;

    Ok((next_input, node_list.spanned(Span::new(start, end))))
}

#[tracable_parser]
pub fn spaced_token_list(input: NomSpan) -> IResult<NomSpan, Spanned<Vec<SpannedToken>>> {
    let start = input.offset;
    let (input, pre_ws) = opt(any_space)(input)?;
    let (input, items) = token_list(input)?;
    let (input, post_ws) = opt(any_space)(input)?;
    let end = input.offset;

    let mut out = vec![];

    if let Some(pre_ws) = pre_ws {
        out.extend(pre_ws)
    }
    out.extend(items.item);
    if let Some(post_ws) = post_ws {
        out.extend(post_ws)
    }

    Ok((input, out.spanned(Span::new(start, end))))
}

fn is_external_word_char(c: char) -> bool {
    match c {
        ';' | '|' | '"' | '\'' | '$' | '(' | ')' | '[' | ']' | '{' | '}' | '`' => false,
        other if other.is_whitespace() => false,
        _ => true,
    }
}

/// These characters appear in globs and not bare words
fn is_glob_specific_char(c: char) -> bool {
    c == '*' || c == '?'
}

fn is_start_glob_char(c: char) -> bool {
    is_start_file_char(c) || is_glob_specific_char(c) || c == '.'
}

fn is_glob_char(c: char) -> bool {
    is_file_char(c) || is_glob_specific_char(c)
}

fn is_dot(c: char) -> bool {
    c == '.'
}

fn is_path_separator(c: char) -> bool {
    match c {
        '\\' | '/' | ':' => true,
        _ => false,
    }
}

fn is_start_file_char(c: char) -> bool {
    match c {
        '+' => false,
        _ if c.is_alphanumeric() => true,
        '\\' => true,
        '/' => true,
        '_' => true,
        '-' => true,
        '~' => true,
        '.' => true,
        _ => false,
    }
}

fn is_file_char(c: char) -> bool {
    match c {
        '+' => true,
        _ if c.is_alphanumeric() => true,
        '\\' => true,
        '/' => true,
        '_' => true,
        '-' => true,
        '=' => true,
        '~' => true,
        ':' => true,
        '?' => true,
        _ => false,
    }
}
