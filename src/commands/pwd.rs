use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::commands::Command;
use crate::context::CommandRegistry;
use crate::error::ShellError;
use crate::evaluate::{CallInfo, Value};
use crate::shell::Shell;
use crate::signature::Signature;

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
        call_info: CallInfo,
        input: Option<Vec<Value>>,
        ctrl_c: Arc<AtomicBool>,
        shell: Arc<dyn Shell>,
        registry: &CommandRegistry,
    ) -> Result<Option<Vec<Value>>, ShellError> {
        unimplemented!()
    }
}
