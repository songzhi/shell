use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use serde::Deserialize;

use crate::commands::{Command, RunnableContext};
use crate::context::CommandRegistry;
use crate::error::ShellError;
use crate::evaluate::{CallInfo, Value};
use crate::parser::syntax_shape::SyntaxShape;
use crate::shell::Shell;
use crate::signature::Signature;

#[derive(Deserialize)]
pub struct CdArgs {
    pub dst: Option<PathBuf>,
}

pub struct Cd;

impl Command for Cd {
    fn name(&self) -> &str {
        "cd"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .optional(
                "destination",
                SyntaxShape::Path,
                "the directory to change to",
            )
            .desc(self.usage())
    }

    fn usage(&self) -> &str {
        "Change to a new path."
    }

    fn run(
        &self,
        call_info: CallInfo,
        input: Option<Vec<Value>>,
        ctrl_c: Arc<AtomicBool>,
        shell: Arc<dyn Shell>,
        _registry: &CommandRegistry,
    ) -> Result<Option<Vec<Value>>, ShellError> {
        call_info.process(&shell, ctrl_c, cd, input)?.run()
    }
}

fn cd(args: CdArgs, ctx: &RunnableContext) -> Result<Option<Vec<Value>>, ShellError> {
    ctx.shell.cd(args)
}
