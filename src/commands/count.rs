use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use num_bigint::BigInt;

use crate::commands::Command;
use crate::context::CommandRegistry;
use crate::error::ShellError;
use crate::evaluate::{CallInfo, Value};
use crate::shell::Shell;

pub struct Count;

impl Command for Count {
    fn name(&self) -> &str {
        "count"
    }

    fn usage(&self) -> &str {
        "Show the total number of rows."
    }

    fn run(
        &self,
        _call_info: CallInfo,
        input: Option<Vec<Value>>,
        _ctrl_c: Arc<AtomicBool>,
        _shell: Arc<dyn Shell>,
        _registry: &CommandRegistry,
    ) -> Result<Option<Vec<Value>>, ShellError> {
        Ok(Some(vec![Value::Int(BigInt::from(
            input.map(|v| v.len()).unwrap_or(0),
        ))]))
    }
}
