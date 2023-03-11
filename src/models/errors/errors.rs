use crossterm::style::Stylize;

use super::{file_error::FileError, parsing_error::ParsingError};
use crate::reset_terminal;

pub enum Errors {
    ParsingError(ParsingError),
    FileError(FileError),
}

impl Errors {
    fn throw_error(err_type: &str, err: String) -> ! {
        eprintln!("{}: {}", format!("{} Error", err_type).red().bold(), err);
        reset_terminal().unwrap();
        std::process::exit(1);
    }

    pub fn throw_parsing_error(err: ParsingError) -> ! {
        Errors::throw_error("Parsing", err.to_string())
    }

    pub fn throw_file_error(err: FileError) -> ! {
        Errors::throw_error("File", err.to_string())
    }
}
