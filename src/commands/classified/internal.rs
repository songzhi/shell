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
    unimplemented!()
}
