pub use error::Error;
use structopt::StructOpt;
use std::path::PathBuf;

mod error;

#[derive(Debug, Clone, Copy)]
pub enum YotType {
    Y8 = 1,
    Y16 = 2,
    Y32 = 4,
    Y64 = 8,
}

#[derive(Debug, StructOpt)]
pub struct Config {
    /// Yot type
    #[structopt(name = "YOT TYPE", parse(try_from_str = parse_yot_type))]
    pub yot_type: YotType,
    /// Yot Assembly source file path
    #[structopt(name = "SOURCE FILE", parse(from_os_str))]
    pub source_path: PathBuf,
    /// Output binary file path
    #[structopt(name = "OUTPUT FILE", parse(from_os_str))]
    pub output_path: PathBuf,
    /// Initial data stack pointer
    #[structopt(long = "sp")]
    pub initial_stack_pointer: u64,
    /// Binary size
    #[structopt(short = "s", long = "exact-size")]
    pub exact_binary_size: Option<usize>,
}

fn parse_yot_type(input: &str) -> Result<YotType, Error> {
    match input {
        "yot-8" => Ok(YotType::Y8),
        "yot-16" => Ok(YotType::Y16),
        "yot-32" => Ok(YotType::Y32),
        "yot-64" => Ok(YotType::Y64),
        _ => Err(Error::YotTypeInvalid),
    }
}
