use crate::context::Context;
use crate::error::ShellError;
use crate::evaluate::Value;
use crate::parser::command::classified::internal::InternalCommand;

pub(crate) fn run_internal_command(
    command: InternalCommand,
    context: &mut Context,
    input: Option<Vec<Value>>,
    source: &str,
) -> Result<Option<Vec<Value>>, ShellError> {
    let internal_command = context.expect_command(command.name.as_str())?;
    context.run_command(internal_command, command.args, source, input)
}
