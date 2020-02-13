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
pub struct CopyArgs {
    pub src: PathBuf,
    pub dst: PathBuf,
}

pub struct Cp;

impl Command for Cp {
    fn name(&self) -> &str {
        "cp"
    }

    fn usage(&self) -> &str {
        "Copy files."
    }
    fn signature(&self) -> Signature {
        Signature::build("cp")
            .required("src", SyntaxShape::Pattern, "the place to copy from")
            .required("dst", SyntaxShape::Path, "the place to copy to")
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
        call_info.process(&shell, ctrl_c, cp, input)?.run()
    }
}

fn cp(args: CopyArgs, ctx: &RunnableContext) -> Result<Option<Vec<Value>>, ShellError> {
    ctx.shell.cp(args)
}
