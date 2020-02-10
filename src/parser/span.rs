use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Span {
    start: usize,
    end: usize,
}

pub fn span_for_spanned_list(mut iter: impl Iterator<Item = Span>) -> Span {
    let first = iter.next();

    let first = match first {
        None => return Span::unknown(),
        Some(first) => first,
    };

    let last = iter.last();

    match last {
        None => first,
        Some(last) => first.until(last),
    }
}

impl From<&Span> for Span {
    fn from(span: &Span) -> Span {
        *span
    }
}

impl From<Option<Span>> for Span {
    fn from(input: Option<Span>) -> Span {
        match input {
            None => Span::new(0, 0),
            Some(span) => span,
        }
    }
}

impl From<(usize, usize)> for Span {
    fn from((start, end): (usize, usize)) -> Span {
        Span { start, end }
    }
}

impl Span {
    pub fn unknown() -> Span {
        Span::new(0, 0)
    }

    pub fn new(start: usize, end: usize) -> Span {
        assert!(
            end >= start,
            "Can't create a Span whose end < start, start={}, end={}",
            start,
            end
        );

        Span { start, end }
    }

    pub fn for_char(pos: usize) -> Span {
        Span {
            start: pos,
            end: pos + 1,
        }
    }

    pub fn contains(&self, pos: usize) -> bool {
        self.start <= pos && self.end >= pos
    }

    pub fn since(&self, other: impl Into<Span>) -> Span {
        let other = other.into();

        Span::new(other.start, self.end)
    }

    pub fn until(&self, other: impl Into<Span>) -> Span {
        let other = other.into();

        Span::new(self.start, other.end)
    }

    pub fn until_option(&self, other: Option<impl Into<Span>>) -> Span {
        match other {
            Some(other) => {
                let other = other.into();

                Span::new(self.start, other.end)
            }
            None => *self,
        }
    }

    pub fn string(&self, source: &str) -> String {
        self.slice(source).to_string()
    }

    pub fn spanned_slice<'a>(&self, source: &'a str) -> Spanned<&'a str> {
        self.slice(source).spanned(*self)
    }

    pub fn spanned_string(&self, source: &str) -> Spanned<String> {
        self.slice(source).to_string().spanned(*self)
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn is_unknown(&self) -> bool {
        self.start == 0 && self.end == 0
    }

    pub fn slice<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..self.end]
    }
}

/// A wrapper type that attaches a Span to a value
#[derive(new, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Spanned<T> {
    pub span: Span,
    pub item: T,
}

impl<T> Spanned<T> {
    /// Allows mapping over a Spanned value
    pub fn map<U>(self, input: impl FnOnce(T) -> U) -> Spanned<U> {
        let span = self.span;

        let mapped = input(self.item);
        mapped.spanned(span)
    }
}

impl Spanned<String> {
    /// Iterates over the contained String
    pub fn items<'a, U>(
        items: impl Iterator<Item = &'a Spanned<String>>,
    ) -> impl Iterator<Item = &'a str> {
        items.map(|item| &item.item[..])
    }
}

impl Spanned<String> {
    /// Borrows the contained String
    pub fn borrow_spanned(&self) -> Spanned<&str> {
        let span = self.span;
        self.item.as_str().spanned(span)
    }
}

pub trait SpannedItem: Sized {
    /// Converts a value into a Spanned value
    fn spanned(self, span: impl Into<Span>) -> Spanned<Self> {
        Spanned {
            item: self,
            span: span.into(),
        }
    }

    /// Converts a value into a Spanned value, using an unknown Span
    fn spanned_unknown(self) -> Spanned<Self> {
        Spanned {
            item: self,
            span: Span::unknown(),
        }
    }
}

impl<T> SpannedItem for T {}

impl<T> std::ops::Deref for Spanned<T> {
    type Target = T;

    /// Shorthand to deref to the contained value
    fn deref(&self) -> &T {
        &self.item
    }
}

pub trait HasSpan {
    fn span(&self) -> Span;
}

impl<T, E> HasSpan for Result<T, E>
where
    T: HasSpan,
{
    fn span(&self) -> Span {
        match self {
            Result::Ok(val) => val.span(),
            Result::Err(_) => Span::unknown(),
        }
    }
}

impl<T> HasSpan for Spanned<T> {
    fn span(&self) -> Span {
        self.span
    }
}
