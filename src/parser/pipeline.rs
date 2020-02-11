use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::parser::span::HasSpan;

use super::{Span, Spanned, SpannedToken};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, new, Serialize, Deserialize)]
pub struct Pipeline {
    pub parts: Vec<PipelineElement>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct PipelineElement {
    pub pipe: Option<Span>,
    pub tokens: Spanned<Vec<SpannedToken>>,
}

impl HasSpan for PipelineElement {
    fn span(&self) -> Span {
        match self.pipe {
            Option::None => self.tokens.span,
            Option::Some(pipe) => pipe.until(self.tokens.span),
        }
    }
}

impl PipelineElement {
    pub fn new(pipe: Option<Span>, tokens: Spanned<Vec<SpannedToken>>) -> PipelineElement {
        PipelineElement { pipe, tokens }
    }

    pub fn tokens(&self) -> &[SpannedToken] {
        &self.tokens.item
    }
}
