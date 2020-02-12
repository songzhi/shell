use std::io::Stdout;
use std::process::Stdio;

use crate::context::CommandRegistry;
use crate::error::ShellError;
use crate::evaluate::CallInfo;
use crate::signature::Signature;

pub mod classified;

pub trait Command: Send + Sync {
    fn name(&self) -> &str;

    fn signature(&self) -> Signature {
        Signature::new(self.name()).desc(self.usage())
    }

    fn usage(&self) -> &str;

    fn run(
        &self,
        call_info: CallInfo,
        registry: &CommandRegistry,
        input: Stdio,
    ) -> Result<Stdout, ShellError>;

    fn is_binary(&self) -> bool {
        false
    }
}

pub type BoxedCommand = Box<dyn Command>;
