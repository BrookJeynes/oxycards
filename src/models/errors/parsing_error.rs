use core::fmt;

pub enum ParsingError {
    NoCardType,
    NoQuestion,
    NoContent,
    IncorrectDivider,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsingError::NoContent => {
                write!(f, "No available content to parse for one or more cards.")
            }
            ParsingError::NoQuestion => write!(f, "No question provided for one or more cards. The question must be prefixed with a hashtag (#)"),
            ParsingError::NoCardType => {
                write!(f, "One or more cards have not specified their card type.\nA list of all supported card types can be found here: https://brookjeynes.github.io/quiz-rs/cards")
            }
            ParsingError::IncorrectDivider => {
                write!(f, "One or more cards have an incorrect divider (---)")
            }
        }
    }
}
