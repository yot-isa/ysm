use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use error::Error;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

mod error;
mod instruction;
mod span;
mod tokenizer;

#[derive(Debug, StructOpt)]
pub struct Config {
    /// Yot Assembly source file path
    #[structopt(name = "SOURCE FILE", parse(from_os_str))]
    pub source_path: PathBuf,
}

fn main() -> Result<(), codespan_reporting::files::Error> {
    let config = Config::from_args();
    let mut files: SimpleFiles<String, &str> = SimpleFiles::new();
    let codespan_writer = StandardStream::stderr(ColorChoice::Always);
    let codespan_config = codespan_reporting::term::Config::default();

    let source_contents = match fs::read_to_string(&config.source_path) {
        Ok(source_contents) => source_contents,
        Err(err) => {
            let diagnostic = Diagnostic::error().with_message(format!(
                "couldn't read {}: {}",
                config.source_path.to_str().unwrap(),
                err
            ));
            term::emit(
                &mut codespan_writer.lock(),
                &codespan_config,
                &files,
                &diagnostic,
            )?;
            return Ok(());
        }
    };

    // let file_id = files.add(config.source_path, source_contents);

    tokenizer::tokenize(&source_contents);

    Ok(())
}
