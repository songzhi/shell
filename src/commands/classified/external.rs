use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use crate::context::Context;
use crate::error::ShellError;
use crate::evaluate::Value;
use crate::parser::command::classified::external::ExternalCommand;

pub(crate) fn run_external_command(
    command: ExternalCommand,
    _context: &mut Context,
    input: Option<Vec<Value>>,
    is_last: bool,
) -> Result<Option<Vec<Value>>, ShellError> {
    if !did_find_command(&command.name) {
        return Err(ShellError::runtime_error("Command not found"));
    }
    let args = command
        .args
        .iter()
        .map(|arg| {
            let arg = expand_tilde(arg.as_str(), dirs::home_dir);

            #[cfg(not(windows))]
            {
                if argument_contains_whitespace(&arg) && argument_is_quoted(&arg) {
                    if let Some(unquoted) = remove_quotes(&arg) {
                        format!(r#""{}""#, unquoted)
                    } else {
                        arg.as_ref().to_string()
                    }
                } else {
                    arg.as_ref().to_string()
                }
            }
            #[cfg(windows)]
            {
                if let Some(unquoted) = remove_quotes(&arg) {
                    unquoted.to_string()
                } else {
                    arg.as_ref().to_string()
                }
            }
        })
        .collect::<Vec<String>>();
    let mut process = {
        #[cfg(windows)]
        {
            let mut process = Command::new("cmd");
            process.arg("/c");
            process.arg(&command.name);
            for arg in args {
                process.arg(&arg);
            }
            process
        }

        #[cfg(not(windows))]
        {
            let cmd_with_args = vec![command.name.clone(), args.join(" ")].join(" ");
            let mut process = Command::new("sh");
            process.arg("-c").arg(cmd_with_args);
            process
        }
    };
    if !is_last {
        process.stdout(Stdio::piped());
    }
    if input.is_some() {
        process.stdin(Stdio::piped());
    }
    if let Ok(mut child) = process.spawn() {
        if let Some(input) = input {
            let mut stdin_write = child
                .stdin
                .take()
                .expect("Internal error: could not get stdin pipe for external command");
            for val in input {
                if let Err(e) = stdin_write.write(val.to_string().as_bytes()) {
                    let message = format!("Unable to write to stdin (error = {})", e);
                    return Err(ShellError::runtime_error(message));
                }
            }
        }
        return if !is_last {
            let stdout = if let Some(stdout) = child.stdout.take() {
                stdout
            } else {
                return Err(ShellError::runtime_error("can't redirect stdout"));
            };
            let mut buf_reader = BufReader::new(stdout);
            let mut results = vec![];
            let mut buf = String::new();
            while let Ok(n) = buf_reader.read_line(&mut buf) {
                if n == 0 {
                    break;
                }
                results.push(Value::String(buf.as_str().to_string()));
                buf.clear();
            }
            Ok(Some(results))
        } else {
            child
                .wait()
                .map_err(|_| ShellError::runtime_error("command's not running"))
                .map(|_| None)
        };
    }
    Ok(None)
}

fn did_find_command(name: &str) -> bool {
    #[cfg(not(windows))]
    {
        which::which(name).is_ok()
    }

    #[cfg(windows)]
    {
        if which::which(name).is_ok() {
            true
        } else {
            let cmd_builtins = [
                "call", "cls", "color", "date", "dir", "echo", "find", "hostname", "pause",
                "start", "time", "title", "ver", "copy", "mkdir", "rename", "rd", "rmdir", "type",
            ];

            cmd_builtins.contains(&name)
        }
    }
}

fn expand_tilde<SI: ?Sized, P, HD>(input: &SI, home_dir: HD) -> std::borrow::Cow<str>
where
    SI: AsRef<str>,
    P: AsRef<std::path::Path>,
    HD: FnOnce() -> Option<P>,
{
    shellexpand::tilde_with_context(input, home_dir)
}

pub fn argument_contains_whitespace(argument: &str) -> bool {
    argument.chars().any(|c| c.is_whitespace())
}

fn argument_is_quoted(argument: &str) -> bool {
    if argument.len() < 2 {
        return false;
    }

    (argument.starts_with('"') && argument.ends_with('"')
        || (argument.starts_with('\'') && argument.ends_with('\'')))
}

#[allow(unused)]
fn add_quotes(argument: &str) -> String {
    format!("'{}'", argument)
}

fn remove_quotes(argument: &str) -> Option<&str> {
    if !argument_is_quoted(argument) {
        return None;
    }

    let size = argument.len();

    Some(&argument[1..size - 1])
}
