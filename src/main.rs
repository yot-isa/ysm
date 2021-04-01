use structopt::StructOpt;
use span::{Span, Spanned};
use tokenizer::token::{Token, DataLiteral};
use reporter::{Reporter, Report};
use argument_parser::{Config, YotType};

mod argument_parser;
mod emitter;
mod error;
mod instruction;
mod reader;
mod reporter;
mod span;
mod tokenizer;
mod writer;

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

    let tokens: Vec<Spanned<Token>> = match tokenizer::tokenize(&source_contents, file_id) {
        Ok(tokens) => tokens,
        Err(errs) => {
            for err in errs.iter() {
                err.report(&reporter);
            }
            return;
        }
    };

    let binary: Vec<u8> = match emitter::emit(
        &tokens,
        config.yot_type,
        config.initial_data_stack_pointer,
        config.initial_address_stack_pointer,
    ) {
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
