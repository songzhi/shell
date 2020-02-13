use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use serde::Deserialize;

pub use ls::Ls;

use crate::context::CommandRegistry;
use crate::deserializer::ConfigDeserializer;
use crate::error::ShellError;
use crate::evaluate::{CallInfo, Value};
use crate::shell::Shell;
use crate::signature::Signature;

pub mod classified;
pub mod ls;

pub trait Command: Send + Sync {
    fn name(&self) -> &str;

    fn signature(&self) -> Signature {
        Signature::new(self.name()).desc(self.usage())
    }

    fn usage(&self) -> &str;

    fn run(
        &self,
        call_info: CallInfo,
        input: Option<Vec<Value>>,
        ctrl_c: Arc<AtomicBool>,
        shell: Arc<dyn Shell>,
    ) -> Result<Option<Vec<Value>>, ShellError>;

    fn is_binary(&self) -> bool {
        false
    }
}

pub type BoxedCommand = Arc<dyn Command>;

pub struct RunnableContext {
    pub input: Option<Vec<Value>>,
    pub shell: Arc<dyn Shell>,
    pub ctrl_c: Arc<AtomicBool>,
}

pub struct RunnableArgs<T> {
    args: T,
    context: RunnableContext,
    callback: fn(T, &RunnableContext) -> Result<Option<Vec<Value>>, ShellError>,
}

impl<T> RunnableArgs<T> {
    pub fn run(self) -> Result<Option<Vec<Value>>, ShellError> {
        (self.callback)(self.args, &self.context)
    }
}

impl CallInfo {
    pub(crate) fn process<'de, T: Deserialize<'de>>(
        &self,
        shell: &Arc<dyn Shell>,
        ctrl_c: Arc<AtomicBool>,
        callback: fn(T, &RunnableContext) -> Result<Option<Vec<Value>>, ShellError>,
        input: Option<Vec<Value>>,
    ) -> Result<RunnableArgs<T>, ShellError> {
        let mut deserializer = ConfigDeserializer::from_call_info(self.clone());
        Ok(RunnableArgs {
            args: T::deserialize(&mut deserializer)?,
            context: RunnableContext {
                shell: shell.clone(),
                ctrl_c,
                input,
            },
            callback,
        })
    }
}
