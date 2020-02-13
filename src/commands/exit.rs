use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::commands::Command;
use crate::error::ShellError;
use crate::evaluate::{CallInfo, Value};
use crate::parser::syntax_shape::SyntaxShape;
use crate::shell::Shell;
use crate::signature::Signature;

pub struct Exit;

impl Command for Exit {
    fn name(&self) -> &str {
        "exit"
    }

    fn usage(&self) -> &str {
        "Exit the current shell (or all shells)"
    }
    fn signature(&self) -> Signature {
        Signature::build("exit")
    }
    fn run(
        &self,
        _call_info: CallInfo,
        _input: Option<Vec<Value>>,
        _ctrl_c: Arc<AtomicBool>,
        _shell: Arc<dyn Shell>,
    ) -> Result<Option<Vec<Value>>, ShellError> {
        // TODO: save history
        std::process::exit(0);
    }
}
