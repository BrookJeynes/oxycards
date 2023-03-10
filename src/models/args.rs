use std::path::Path;

use clap::Parser;

use super::{errors::file_error::FileError, file_type::FileType};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Path to a quiz md file
    #[arg(short, long)]
    pub path: String,
}

impl Args {
    pub fn validate_file(file: &Path) -> Result<(), FileError> {
        match file.extension() {
            Some(extension) => {
                if let Some(_) = FileType::from_osstr(extension) {
                    return Ok(());
                }
            }
            None => return Err(FileError::InvalidFileType),
        }

        Err(FileError::InvalidFileType)
    }
}
