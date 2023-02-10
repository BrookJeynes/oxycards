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

impl Card {
    pub fn validate_answer(&mut self) -> UserAnswer {
        match self {
            Self::FlashCard(card) => card.validate_answer(),
            Self::MultipleChoice(card) => card.validate_answer(),
            Self::MultipleAnswer(card) => card.validate_answer(),
            Self::FillInTheBlanks(card) => card.validate_answer(),
            Self::Order(card) => card.validate_answer(),
        }
    }

    pub fn check_answered(&mut self) -> bool {
        match self {
            Self::MultipleChoice(card) => card.check_answered(),
            Self::Order(card) => card.check_answered(),
            Self::MultipleAnswer(card) => card.check_answered(),
            Self::FlashCard(card) => card.check_answered(),
            Self::FillInTheBlanks(card) => card.check_answered(),
        }
    }

    pub fn instructions(&self) -> String {
        match self {
            Self::MultipleChoice(card) => card.instructions(),
            Self::Order(card) => card.instructions(),
            Self::MultipleAnswer(card) => card.instructions(),
            Self::FlashCard(card) => card.instructions(),
            Self::FillInTheBlanks(card) => card.instructions(),
        }
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
