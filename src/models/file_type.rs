use core::fmt;
use std::ffi::OsStr;

#[derive(PartialEq)]
pub enum FileType {
    Markdown,
}

impl FileType {
    pub fn from_osstr(file_extension: &OsStr) -> Option<Self> {
        if file_extension == "md" {
            return Some(FileType::Markdown);
        }

        None
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileType::Markdown => write!(f, "md"),
        }
    }
}
