use external::ExternalCommand;
use internal::InternalCommand;

use crate::parser::span::{HasSpan, Span};

pub mod external;
pub mod internal;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ClassifiedCommand {
    Internal(InternalCommand),
    External(ExternalCommand),
}

#[derive(Debug, Clone)]
pub struct Commands {
    pub list: Vec<ClassifiedCommand>,
    pub span: Span,
}

impl std::ops::Deref for Commands {
    type Target = [ClassifiedCommand];

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl HasSpan for ClassifiedCommand {
    fn span(&self) -> Span {
        match self {
            ClassifiedCommand::Internal(command) => command.span(),
            ClassifiedCommand::External(command) => command.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClassifiedPipeline {
    pub commands: Commands,
}

impl ClassifiedPipeline {
    pub fn commands(list: Vec<ClassifiedCommand>, span: impl Into<Span>) -> ClassifiedPipeline {
        ClassifiedPipeline {
            commands: Commands {
                list,
                span: span.into(),
            },
        }
    }
}

impl HasSpan for ClassifiedPipeline {
    fn span(&self) -> Span {
        self.commands.span
    }
}
