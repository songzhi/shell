use rustyline::error::ReadlineError;

use crate::context::Context;
use crate::error::ShellError;
use crate::parser;
use crate::parser::command::classified::ClassifiedPipeline;
use crate::parser::token::SpannedToken;

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
        Ok(line) if line.trim() == "" => LineResult::Success(line.clone()),
        Ok(line) => {
            let line = chomp_newline(line);
            let result = match parser::parse(&line) {
                Err(err) => {
                    return LineResult::Error(line.to_string(), err);
                }
                Ok(val) => val,
            };
            let pipeline = classify_pipeline(&result, ctx, line);

            LineResult::Success(line.to_string().clone())
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
) -> ClassifiedPipeline {
    let pipeline_list = vec![pipeline.clone()];
    unimplemented!()
}
