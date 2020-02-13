use std::sync::atomic::Ordering;
use std::sync::Arc;

use rustyline::error::ReadlineError;
use rustyline::{
    self, config::Configurer, config::EditMode, At, Cmd, ColorMode, CompletionType, Config, Editor,
    KeyPress, Movement, Word,
};

use crate::commands::classified::pipeline::run_pipeline;
use crate::commands::{BoxedCommand, Command};
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

pub fn cli() -> Result<(), ShellError> {
    let mut context = create_default_context();
    let config = Config::builder().color_mode(ColorMode::Forced).build();
    let mut rl: Editor<()> = Editor::with_config(config);
    // add key bindings to move over a whole word with Ctrl+ArrowLeft and Ctrl+ArrowRight
    rl.bind_sequence(
        KeyPress::ControlLeft,
        Cmd::Move(Movement::BackwardWord(1, Word::Vi)),
    );
    rl.bind_sequence(
        KeyPress::ControlRight,
        Cmd::Move(Movement::ForwardWord(1, At::AfterEnd, Word::Vi)),
    );
    #[cfg(windows)]
    {
        let _ = ansi_term::enable_ansi_support();
    }
    let mut ctrlcbreak = false;
    loop {
        if context.ctrl_c.load(Ordering::SeqCst) {
            context.ctrl_c.store(false, Ordering::SeqCst);
            continue;
        }
        let cwd = std::env::current_dir().expect("can't get current dir");
        let colored_prompt = format!("\x1b[32m{}\x1b[m> ", cwd.to_string_lossy().to_string());
        let prompt = {
            if let Ok(bytes) = strip_ansi_escapes::strip(&colored_prompt) {
                String::from_utf8_lossy(&bytes).to_string()
            } else {
                "> ".to_string()
            }
        };
        let mut initial_command = Some(String::new());
        let mut readline = Err(ReadlineError::Eof);
        while let Some(ref cmd) = initial_command {
            readline = rl.readline_with_initial(&prompt, (&cmd, ""));
            initial_command = None;
        }
        let line = process_line(readline, &mut context, false);
        match line {
            LineResult::Success(_) => {}
            LineResult::Error(_, _) => {}
            LineResult::CtrlC => {
                if ctrlcbreak {
                    std::process::exit(0);
                } else {
                    ctrlcbreak = true;
                    continue;
                }
            }
            LineResult::Break => {
                break;
            }
        }
        ctrlcbreak = false;
    }
    Ok(())
}

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
        Err(_) => LineResult::Break,
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

fn create_default_context() -> Context {
    let mut context = Context::basic();
    #[inline]
    fn command(c: impl Command + 'static) -> BoxedCommand {
        Arc::new(c)
    }
    {
        use crate::commands::*;
        context.add_commands(vec![command(Ls)])
    }
    context
}
