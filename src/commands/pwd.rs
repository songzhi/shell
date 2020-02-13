use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::commands::Command;
use crate::context::CommandRegistry;
use crate::error::ShellError;
use crate::evaluate::{CallInfo, Value};
use crate::shell::Shell;

pub struct Pwd;

impl Command for Pwd {
    fn name(&self) -> &str {
        "pwd"
    }

    fn usage(&self) -> &str {
        "Output the current working directory."
    }

    fn run(
        &self,
        _call_info: CallInfo,
        _input: Option<Vec<Value>>,
        _ctrl_c: Arc<AtomicBool>,
        shell: Arc<dyn Shell>,
        _registry: &CommandRegistry,
    ) -> Result<Option<Vec<Value>>, ShellError> {
        shell.pwd()
    }
}
