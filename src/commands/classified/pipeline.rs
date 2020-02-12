use crate::commands::classified::external::run_external_command;
use crate::commands::classified::internal::run_internal_command;
use crate::context::Context;
use crate::error::ShellError;
use crate::evaluate::Value;
use crate::parser::command::classified::{ClassifiedCommand, ClassifiedPipeline};

pub(crate) fn run_pipeline(
    pipeline: ClassifiedPipeline,
    ctx: &mut Context,
    mut input: Option<Vec<Value>>,
    line: &str,
) -> Result<Option<Vec<Value>>, ShellError> {
    let mut iter = pipeline.commands.list.into_iter().peekable();
    loop {
        let item = iter.next();
        let next = iter.peek();

        input = match (item, next) {
            (Some(ClassifiedCommand::Internal(command)), _) => {
                run_internal_command(command, ctx, input, line)?
            }
            (Some(ClassifiedCommand::External(command)), None) => {
                run_external_command(command, ctx, input, true)?
            }
            (Some(ClassifiedCommand::External(command)), _) => {
                run_external_command(command, ctx, input, false)?
            }
            _ => break,
        }
    }
    Ok(input)
}
