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

macro_rules! impl_various {
    ($($var:ident),*) => {
        impl Card {
            pub fn validate_answer(&mut self) -> UserAnswer {
                match self {
                    $(Card::$var(card) => card.validate_answer(),)*
                }
            }

            pub fn check_answered(&mut self) -> bool {
                match self {
                    $(Card::$var(card) => card.user_answer != UserAnswer::Undecided,)*
                }
            }

            pub fn instructions(&self) -> String {
                match self {
                    $(Card::$var(card) => card.instructions(),)*
                }
            }
        }

        impl fmt::Display for Card {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(Card::$var(card) => write!(f, "{card}"),)*
                }
            }
        }
    }
}

impl_various!(
    FlashCard,
    MultipleAnswer,
    MultipleChoice,
    FillInTheBlanks,
    Order
);
