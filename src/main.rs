use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use error::Error;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use span::{Location, Span, Spanned, Spanning};
use tokenizer::token::{Token, DataLiteral};
use instruction::YotType;

mod emitter;
mod error;
mod instruction;
mod reader;
mod span;
mod tokenizer;
mod writer;

#[derive(Debug, StructOpt)]
pub struct Config {
    /// Yot Assembly source file path
    #[structopt(name = "SOURCE FILE", parse(from_os_str))]
    pub source_path: PathBuf,
    /// Output binary file path
    #[structopt(name = "OUTPUT FILE", parse(from_os_str))]
    pub output_path: PathBuf,
}

fn main() {
    let config = Config::from_args();
    let mut files: SimpleFiles<String, &str> = SimpleFiles::new();
    let codespan_writer = StandardStream::stderr(ColorChoice::Always);
    let codespan_config = codespan_reporting::term::Config::default();

    let source_contents = match reader::read(&config.source_path) {
        Ok(source_contents) => source_contents,
        Err(err) => {
            let diagnostic = Diagnostic::error().with_message(format!("{}", err));
            term::emit(
                &mut codespan_writer.lock(),
                &codespan_config,
                &files,
                &diagnostic,
            );
            return;
        }
    };

    let file_id = files.add(
        config.source_path.to_string_lossy().to_string(),
        &source_contents,
    );

    let tokens: Vec<Spanned<Token>> = match tokenizer::tokenize(&source_contents) {
        Ok(tokens) => tokens,
        Err(errs) => {
            for err in errs.iter() {
                let diagnostic = Diagnostic::error().with_message(format!("{}", err));
                term::emit(
                    &mut codespan_writer.lock(),
                    &codespan_config,
                    &files,
                    &diagnostic,
                );
            }
            return;
        }
    };

    let binary: Vec<u8> = match emitter::emit(&tokens, YotType::Y8) {
        Ok(binary) => binary,
        Err(()) => {
            return;
        }
    };

    match writer::write(&config.output_path, &binary) {
        Ok(()) => (),
        Err(err) => {
            let diagnostic = Diagnostic::error().with_message(format!("{}", err));
            term::emit(
                &mut codespan_writer.lock(),
                &codespan_config,
                &files,
                &diagnostic,
            );
            return;
        }
    }
}
