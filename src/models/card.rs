use core::fmt;

use super::{
    fill_in_the_blanks::FillInTheBlanks, flashcard::FlashCard, multiple_answer::MultipleAnswer,
    multiple_choice::MultipleChoice, order::Order,
};

pub enum Card {
    FlashCard(FlashCard),
    MultipleChoice(MultipleChoice),
    MultipleAnswer(MultipleAnswer),
    FillInTheBlanks(FillInTheBlanks),
    Order(Order),
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FlashCard(card) => write!(f, "{card}"),
            Self::MultipleChoice(card) => write!(f, "{card}"),
            Self::MultipleAnswer(card) => write!(f, "{card}"),
            Self::FillInTheBlanks(card) => write!(f, "{card}"),
            Self::Order(card) => write!(f, "{card}"),
        }
    }
}
