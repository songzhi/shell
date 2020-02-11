use crate::parser::span::{HasSpan, Span};

pub type ExternalArg = String;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExternalArgs {
    pub list: Vec<ExternalArg>,
    pub span: Span,
}

impl ExternalArgs {
    pub fn iter(&self) -> impl Iterator<Item = &ExternalArg> {
        self.list.iter()
    }
}

impl std::ops::Deref for ExternalArgs {
    type Target = [ExternalArg];

    fn deref(&self) -> &[ExternalArg] {
        &self.list
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExternalCommand {
    pub name: String,
    pub name_span: Span,
    pub args: ExternalArgs,
}

impl HasSpan for ExternalCommand {
    fn span(&self) -> Span {
        self.name_span.until(self.args.span)
    }
}
