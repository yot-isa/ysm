use error::Error;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use span::{Location, Span, Spanned, Spanning};
use tokenizer::token::{Token, DataLiteral};
use instruction::YotType;
use reporter::{Reporter, Report};

mod emitter;
mod error;
mod instruction;
mod reader;
mod reporter;
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
    let mut reporter = Reporter::new();

    let source_contents = match reader::read(&config.source_path) {
        Ok(source_contents) => source_contents,
        Err(err) => {
            err.report(&reporter);
            return;
        }
    };

    let file_id = reporter.add_file(
        config.source_path,
        &source_contents,
    );
    // let file_id = repoter.add_file(
    //     config.source_path.to_string_lossy().to_string(),
    //     &source_contents,
    // );

    let tokens: Vec<Spanned<Token>> = match tokenizer::tokenize(&source_contents) {
        Ok(tokens) => tokens,
        Err(errs) => {
            for err in errs.iter() {
                err.report(&reporter);
            }
            return;
        }
    };

    let binary: Vec<u8> = match emitter::emit(&tokens, YotType::Y8) {
        Ok(binary) => binary,
        Err(errs) => {
            for err in errs.iter() {
                err.report(&reporter);
            }
            return;
        }
    };

    match writer::write(&config.output_path, &binary) {
        Ok(()) => (),
        Err(err) => {
            err.report(&reporter);
            return;
        }
    }
}
