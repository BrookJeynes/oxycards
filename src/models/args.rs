use std::{ffi::OsStr, path::Path};

use clap::Parser;

use crate::FileError;

use super::file_type::FileType;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Path to a quiz md file
    #[arg(short, long)]
    pub path: String,
}

impl Args {
    pub fn validate_file(file: &Path) -> Result<(), FileError> {
        match file.extension().and_then(OsStr::to_str) {
            Some(extension) => {
                if let Some(_) = FileType::from_str(extension) {
                    return Ok(());
                }
            }
            None => return Err(FileError::InvalidFileType),
        }

        Err(FileError::InvalidFileType)
    }
}
