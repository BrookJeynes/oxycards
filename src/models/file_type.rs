use core::fmt;

#[derive(PartialEq)]
pub enum FileType {
    Markdown,
}

impl FileType {
    pub fn from_str(file_extension: &str) -> Option<Self> {
        match file_extension {
            "md" => Some(FileType::Markdown),
            _ => None,
        }
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileType::Markdown => write!(f, "md"),
        }
    }
}
