use derive_new::new;

use crate::parser::hir;
use crate::parser::span::{HasSpan, Span};

#[derive(new, Debug, Clone, Eq, PartialEq)]
pub struct InternalCommand {
    pub name: String,
    pub name_span: Span,
    pub args: hir::Call,
}

impl HasSpan for InternalCommand {
    fn span(&self) -> Span {
        let start = self.name_span;

        start.until(self.args.span)
    }
}
