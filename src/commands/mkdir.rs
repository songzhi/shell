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
pub struct MkdirArgs {
    pub rest: Vec<PathBuf>,
}

pub struct Mkdir;

impl Command for Mkdir {
    fn name(&self) -> &str {
        "mkdir"
    }

    fn usage(&self) -> &str {
        "Make directories, creates intermediary directories as required."
    }
    fn signature(&self) -> Signature {
        Signature::build("mkdir")
            .rest(SyntaxShape::Path, "the name of the path to create")
            .desc(self.usage())
    }
    fn run(
        &self,
        call_info: CallInfo,
        input: Option<Vec<Value>>,
        ctrl_c: Arc<AtomicBool>,
        shell: Arc<dyn Shell>,
        _registry: &CommandRegistry,
    ) -> Result<Option<Vec<Value>>, ShellError> {
        call_info.process(&shell, ctrl_c, mkdir, input)?.run()
    }
}

fn mkdir(args: MkdirArgs, ctx: &RunnableContext) -> Result<Option<Vec<Value>>, ShellError> {
    ctx.shell.mkdir(args)
}
