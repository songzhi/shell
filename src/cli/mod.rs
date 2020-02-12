use rustyline::error::ReadlineError;

use crate::commands::classified::pipeline::run_pipeline;
use crate::context::Context;
use crate::error::{ProximateShellError, ShellError};
use crate::parser;
use crate::parser::command::classified::external::{ExternalArgs, ExternalCommand};
use crate::parser::command::classified::internal::InternalCommand;
use crate::parser::command::classified::{ClassifiedCommand, ClassifiedPipeline, Commands};
use crate::parser::command::parse_command_tail;
use crate::parser::span::HasSpan;
use crate::parser::token::{SpannedToken, Token};

pub mod colors;

enum LineResult {
    Success(String),
    Error(String, ShellError),
    CtrlC,
    Break,
}

fn process_line(
    readline: Result<String, ReadlineError>,
    ctx: &mut Context,
    redirect_stdin: bool,
) -> LineResult {
    match &readline {
        Ok(line) if line.trim().is_empty() => LineResult::Success(line.clone()),
        Ok(line) => {
            let line = chomp_newline(line);
            let result = parser::parse(line)
                .and_then(|pipeline| classify_pipeline(&pipeline, ctx, line))
                .and_then(|pipeline| run_pipeline(pipeline, ctx, None, line));
            match result {
                Ok(output) => {
                    if let Some(output) = output {
                        for val in output {
                            println!("{}", val.to_string());
                        }
                    };
                    LineResult::Success(line.to_string().clone())
                }
                Err(err) => LineResult::Error(line.to_string(), err),
            }
        }
        Err(ReadlineError::Interrupted) => LineResult::CtrlC,
        Err(ReadlineError::Eof) => LineResult::Break,
        Err(err) => LineResult::Break,
    }
}

fn chomp_newline(s: &str) -> &str {
    if s.ends_with('\n') {
        &s[..s.len() - 1]
    } else {
        s
    }
}

pub fn classify_pipeline(
    pipeline: &SpannedToken,
    context: &Context,
    source: &str,
) -> Result<ClassifiedPipeline, ShellError> {
    let span = pipeline.span;
    match &pipeline.item {
        Token::Pipeline(pipeline) => {
            let mut commands = vec![];
            for elem in pipeline.parts.iter() {
                let mut tokens = elem.tokens.item.iter().cloned();
                let head: SpannedToken = tokens.next().expect("expected command");
                let name = match head.item {
                    Token::Bare | Token::GlobPattern | Token::ExternalWord => {
                        head.span.slice(source)
                    }
                    _ => {
                        return Err(ProximateShellError::ParseError(
                            head.span,
                            Some(format!("expected command found: {}", head.item.desc())),
                        )
                        .start());
                    }
                };
                if let Some(command) = context.registry.get_command(name) {
                    let name_span = head.span;
                    let args = if let Some((positional, named)) =
                        parse_command_tail(&command.signature(), &mut tokens, head.span, source)?
                    {
                        parser::hir::Call {
                            head,
                            positional,
                            named,
                            span: elem.span(),
                        }
                    } else {
                        parser::hir::Call {
                            head,
                            positional: None,
                            named: None,
                            span: elem.span(),
                        }
                    };
                    let command = InternalCommand {
                        name: name.to_string(),
                        name_span,
                        args,
                    };
                    commands.push(ClassifiedCommand::Internal(command));
                } else {
                    let args = ExternalArgs::from_tokens(&mut tokens, source, elem.span());
                    let command = ExternalCommand {
                        name: name.to_string(),
                        name_span: head.span,
                        args,
                    };
                    commands.push(ClassifiedCommand::External(command));
                }
            }
            Ok(ClassifiedPipeline {
                commands: Commands {
                    list: commands,
                    span,
                },
            })
        }
        _ => panic!("expected token"),
    }
}
