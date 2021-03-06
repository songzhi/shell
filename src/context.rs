use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use indexmap::IndexMap;
use parking_lot::Mutex;

use crate::commands::BoxedCommand;
use crate::error::ShellError;
use crate::evaluate::call_info::CallInfo;
use crate::evaluate::{evaluate_args, Value};
use crate::parser::hir::Call;
use crate::shell::{FilesystemShell, Shell};
use crate::signature::Signature;

#[derive(Clone, Default)]
pub struct CommandRegistry {
    registry: Arc<Mutex<IndexMap<String, BoxedCommand>>>,
}

impl CommandRegistry {
    pub fn has(&self, name: &str) -> bool {
        let registry = self.registry.lock();
        registry.contains_key(name)
    }
    pub fn get(&self, name: &str) -> Option<Signature> {
        let registry = self.registry.lock();
        registry.get(name).map(|command| command.signature())
    }

    pub(crate) fn empty() -> CommandRegistry {
        CommandRegistry::default()
    }

    pub fn get_command(&self, name: &str) -> Option<BoxedCommand> {
        let registry = self.registry.lock();
        registry.get(name).cloned()
    }

    pub(crate) fn expect_command(&self, name: &str) -> Result<BoxedCommand, ShellError> {
        self.get_command(name)
            .ok_or_else(|| ShellError::runtime_error(format!("Could not load command: {}", name)))
    }

    pub(crate) fn insert(&mut self, name: impl Into<String>, command: BoxedCommand) {
        let mut registry = self.registry.lock();
        registry.insert(name.into(), command);
    }

    pub(crate) fn names(&self) -> Vec<String> {
        let registry = self.registry.lock();
        registry.keys().cloned().collect()
    }
}

#[derive(Clone)]
pub struct Context {
    pub registry: CommandRegistry,
    pub current_errors: Arc<Mutex<Vec<ShellError>>>,
    pub ctrl_c: Arc<AtomicBool>,
    pub(crate) shell: Arc<dyn Shell>,
}

impl Context {
    pub fn basic() -> Self {
        Self {
            registry: CommandRegistry::empty(),
            current_errors: Arc::new(Mutex::new(Vec::new())),
            ctrl_c: Arc::new(AtomicBool::new(false)),
            shell: Arc::new(FilesystemShell::new()),
        }
    }
    pub fn add_commands(&mut self, commands: Vec<BoxedCommand>) {
        for command in commands {
            self.registry.insert(command.name().to_string(), command);
        }
    }

    pub fn get_command(&self, name: &str) -> Option<BoxedCommand> {
        self.registry.get_command(name)
    }

    pub(crate) fn expect_command(&self, name: &str) -> Result<BoxedCommand, ShellError> {
        self.registry.expect_command(name)
    }
    pub(crate) fn run_command(
        &mut self,
        command: BoxedCommand,
        args: Call,
        source: &str,
        input: Option<Vec<Value>>,
    ) -> Result<Option<Vec<Value>>, ShellError> {
        let call_info = CallInfo {
            args: evaluate_args(args, command.clone(), &self.registry, source)?,
        };
        command.run(
            call_info,
            input,
            self.ctrl_c.clone(),
            self.shell.clone(),
            &self.registry,
        )
    }
}
