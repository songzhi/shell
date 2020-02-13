use std::collections::VecDeque;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use serde::Deserialize;

use crate::commands::Command;
use crate::context::CommandRegistry;
use crate::error::ShellError;
use crate::evaluate::{CallInfo, Value};
use crate::parser::syntax_shape::SyntaxShape;
use crate::shell::Shell;
use crate::signature::{NamedType, PositionalType, Signature};

#[derive(Deserialize)]
pub struct HelpArgs {
    pub command: Option<String>,
}

pub struct Help;

impl Command for Help {
    fn name(&self) -> &str {
        "help"
    }

    fn signature(&self) -> Signature {
        Signature::build("help")
            .rest(SyntaxShape::Any, "the name of command(s) to get help on")
            .desc(self.usage())
    }

    fn usage(&self) -> &str {
        "Display help information about commands."
    }

    fn run(
        &self,
        call_info: CallInfo,
        _input: Option<Vec<Value>>,
        _ctrl_c: Arc<AtomicBool>,
        _shell: Arc<dyn Shell>,
        registry: &CommandRegistry,
    ) -> Result<Option<Vec<Value>>, ShellError> {
        match call_info.args.nth(0) {
            Some(Value::String(s)) => {
                if s == "commands" {
                    let mut help = VecDeque::new();
                    let mut sorted_names = registry.names();
                    sorted_names.sort();
                    for name in sorted_names.iter() {
                        if let Some(command) = registry.get_command(name) {
                            help.push_back(format!("{}: {}", name, command.signature().usage))
                        }
                    }
                    Ok(Some(help.into_iter().map(Value::String).collect()))
                } else {
                    let command = registry.expect_command(s)?;
                    let signature = command.signature();
                    Ok(Some(get_help(s, signature).into()))
                }
            }
            _ => {
                let msg = r#"Welcome to Li's shell.

Here are some tips to help you get started.
  * help commands - list all available commands
  * help <command name> - display help about a particular command"#;
                Ok(Some(vec![Value::String(msg.to_string())]))
            }
        }
    }
}

pub(crate) fn get_help(_cmd_name: &str, cmd_sig: Signature) -> impl Into<Vec<Value>> {
    let mut help = VecDeque::new();
    let mut long_desc = String::new();

    long_desc.push_str(&cmd_sig.usage);
    long_desc.push_str("\n");

    let signature = cmd_sig;

    let mut one_liner = String::new();
    one_liner.push_str(&signature.name);
    one_liner.push_str(" ");

    for positional in &signature.positional {
        match &positional.0 {
            PositionalType::Mandatory(name, _m) => {
                one_liner.push_str(&format!("<{}> ", name));
            }
            PositionalType::Optional(name, _o) => {
                one_liner.push_str(&format!("({}) ", name));
            }
        }
    }

    if signature.rest_positional.is_some() {
        one_liner.push_str(" ...args");
    }

    if !signature.named.is_empty() {
        one_liner.push_str("{flags} ");
    }

    long_desc.push_str(&format!("\nUsage:\n  > {}\n", one_liner));

    if !signature.positional.is_empty() || signature.rest_positional.is_some() {
        long_desc.push_str("\nparameters:\n");
        for positional in signature.positional {
            match positional.0 {
                PositionalType::Mandatory(name, _m) => {
                    long_desc.push_str(&format!("  <{}> {}\n", name, positional.1));
                }
                PositionalType::Optional(name, _o) => {
                    long_desc.push_str(&format!("  ({}) {}\n", name, positional.1));
                }
            }
        }

        if let Some(rest_positional) = signature.rest_positional {
            long_desc.push_str(&format!("  ...args: {}\n", rest_positional.1));
        }
    }
    if !signature.named.is_empty() {
        long_desc.push_str("\nflags:\n");
        for (flag, ty) in signature.named {
            match ty.0 {
                NamedType::Switch => {
                    long_desc.push_str(&format!(
                        "  --{}{} {}\n",
                        flag,
                        if !ty.1.is_empty() { ":" } else { "" },
                        ty.1
                    ));
                }
                NamedType::Mandatory(m) => {
                    long_desc.push_str(&format!(
                        "  --{} <{}> (required parameter){} {}\n",
                        flag,
                        m,
                        if !ty.1.is_empty() { ":" } else { "" },
                        ty.1
                    ));
                }
                NamedType::Optional(o) => {
                    long_desc.push_str(&format!(
                        "  --{} <{}>{} {}\n",
                        flag,
                        o,
                        if !ty.1.is_empty() { ":" } else { "" },
                        ty.1
                    ));
                }
            }
        }
    }

    help.push_back(Value::String(long_desc));
    help
}
