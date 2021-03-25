use codespan_reporting;
use std::path::PathBuf;

pub struct Reporter<'a> {
    pub files: codespan_reporting::files::SimpleFiles<String, &'a str>,
    pub writer: codespan_reporting::term::termcolor::StandardStream,
    pub config: codespan_reporting::term::Config,
}

impl<'a> Reporter<'a> {
    pub fn new() -> Reporter<'a> {
        Reporter {
            files: codespan_reporting::files::SimpleFiles::new(),
            writer: codespan_reporting::term::termcolor::StandardStream::stderr(codespan_reporting::term::termcolor::ColorChoice::Always),
            config: codespan_reporting::term::Config::default(),
        }
    }

    pub fn add_file(&mut self, file_path: PathBuf, file_contents: &'a str) {
        self.files.add(file_path.to_string_lossy().to_string(), file_contents);
    }

    pub fn write(&self, diagnostic: Diagnostic) {
        let codespan_diagnostic = codespan_reporting::diagnostic::Diagnostic::error()
            .with_message(diagnostic.message);
        codespan_reporting::term::emit(
            &mut self.writer.lock(),
            &self.config,
            &self.files,
            &codespan_diagnostic,
        );
    }
}

pub struct Diagnostic {
    pub message: String,
}

pub trait Report {
    fn report(&self, r: &Reporter);
}
