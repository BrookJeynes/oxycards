use core::fmt;

use crate::UserAnswer;

use super::card_types::{
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

pub trait BaseCard {
    fn instructions(&self) -> String;

    fn validate_answer(&mut self) -> UserAnswer;

    fn check_answered(&self) -> bool;
}


impl Card {
    pub fn validate_answer(&mut self) -> UserAnswer {
        match self {
            Self::FlashCard(card) => card.validate_answer(),
            Self::MultipleChoice(card) => card.validate_answer(),
            Self::MultipleAnswer(card) => card.validate_answer(),
            // Self::FillInTheBlanks(card) => card.validate_answer(),
            Self::Order(card) => card.validate_answer(),
            _ => UserAnswer::Undecided
        }
    }

    pub fn check_answered(&mut self) -> bool {
        match self {
            Self::MultipleChoice(card) => card.check_answered(),
            Self::Order(card) => card.check_answered(),
            Self::MultipleAnswer(card) => card.check_answered(),
            Self::FlashCard(card) => card.check_answered(),
            _ => false
        }
    }

// fn test_traits<T>(card: T) where T: BaseCard {
//     card.check_answered
// }
    pub fn instructions<T>(&self) -> String where T: BaseCard {
        self.instructions::<T>()
    }
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
