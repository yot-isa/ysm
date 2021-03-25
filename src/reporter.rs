use codespan_reporting;
use std::path::PathBuf;
use super::Span;

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
            config: codespan_reporting::term::Config {
                display_style: codespan_reporting::term::DisplayStyle::Rich,
                tab_width: 2,
                #[cfg(windows)]
                styles: with_blue(codespan_reporting::term::termcolor::Color::Cyan),
                #[cfg(not(windows))]
                styles: with_blue(codespan_reporting::term::termcolor::Color::Blue),
                chars: codespan_reporting::term::Chars::default(),
                start_context_lines: 3,
                end_context_lines: 1,
            },
        }
    }

    pub fn add_file(&mut self, file_path: PathBuf, file_contents: &'a str) -> usize {
        self.files.add(file_path.to_string_lossy().to_string(), file_contents)
    }

    pub fn write(&self, diagnostic: Diagnostic) {
        let codespan_diagnostic = codespan_reporting::diagnostic::Diagnostic::error()
            .with_message(diagnostic.message)
            .with_labels(diagnostic.labels.iter().map(|label: &Label| -> codespan_reporting::diagnostic::Label<usize> {
                match label.style {
                    LabelStyle::Primary => codespan_reporting::diagnostic::Label::primary(
                        label.span.file_id,
                        label.span.from.offset..label.span.to.offset
                    ).with_message(&label.message),
                    LabelStyle::Secondary => codespan_reporting::diagnostic::Label::secondary(
                        label.span.file_id,
                        label.span.from.offset..label.span.to.offset
                    ).with_message(&label.message),
                }
            }).collect());
        let _ = codespan_reporting::term::emit(
            &mut self.writer.lock(),
            &self.config,
            &self.files,
            &codespan_diagnostic,
        );
    }
}

pub struct Diagnostic {
    pub message: String,
    pub labels: Vec<Label>,
}

pub enum LabelStyle {
    Primary,
    Secondary,
}

pub struct Label {
    pub style: LabelStyle,
    pub span: Span,
    pub message: String,
}

pub trait Report {
    fn report(&self, r: &Reporter);
}

fn with_blue(blue: codespan_reporting::term::termcolor::Color) -> codespan_reporting::term::Styles {
    use codespan_reporting::term::{termcolor::{Color, ColorSpec}, Styles};

    let header = ColorSpec::new().set_bold(true).set_intense(true).clone();

    Styles {
        header_bug: header.clone().set_fg(Some(Color::Red)).clone(),
        header_error: header.clone().set_fg(Some(Color::Red)).clone(),
        header_warning: header.clone().set_fg(Some(Color::Yellow)).clone(),
        header_note: header.clone().set_fg(Some(Color::Green)).clone(),
        header_help: header.clone().set_fg(Some(Color::Cyan)).clone(),
        header_message: header.clone(),

        primary_label_bug: header.clone().set_fg(Some(Color::Red)).clone(),
        primary_label_error: header.clone().set_fg(Some(Color::Red)).clone(),
        primary_label_warning: header.clone().set_fg(Some(Color::Yellow)).clone(),
        primary_label_note: header.clone().set_fg(Some(Color::Green)).clone(),
        primary_label_help: header.clone().set_fg(Some(Color::Cyan)).clone(),
        secondary_label: header.clone().set_fg(Some(blue)).clone(),

        line_number: header.clone().set_fg(Some(blue)).clone(),
        source_border: header.clone().set_fg(Some(blue)).clone(),
        note_bullet: header.clone().set_fg(Some(blue)).clone(),
    }
}
