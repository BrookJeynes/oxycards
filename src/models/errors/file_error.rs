use core::fmt;

pub enum FileError {
    InvalidFileType,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::InvalidFileType => write!(f, "Invalid file type"),
        }
    }
}
