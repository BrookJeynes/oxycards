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
    ($($card_variant:ident),*) => {
        impl Card {
            pub fn validate_answer(&mut self) -> UserAnswer {
                match self {
                    $(Card::$card_variant(card) => card.validate_answer()),*
                }
            }

            pub fn check_answered(&mut self) -> bool {
                match self {
                    $(Card::$card_variant(card) => card.user_answer != UserAnswer::Undecided),*
                }
            }

            pub fn instructions(&self) -> String {
                match self {
                    $(Card::$card_variant(card) => card.instructions()),*
                }
            }
        }

        impl fmt::Display for Card {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(Card::$card_variant(card) => write!(f, "{card}")),*
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


#[derive(Debug)]
pub enum ParsingError {
    NoCardType,
}

impl Card {
    pub fn extract_card_title(content: &String) -> (String, String) {
        // Don't unwrap
        let question = content.lines().nth(0).unwrap()[1..].trim().to_string();
        let content = content.lines().skip(1).collect::<Vec<&str>>().join("\n");

        (question, content)
    }

    pub fn card_parser(content: String) -> Result<Vec<Self>, ParsingError> {
        let cards: Vec<Card> = content
            .split("---")
            .map(|section| {
                let sections = section
                    .trim()
                    .split("\n\n")
                    .filter(|item| !item.is_empty())
                    .collect::<Vec<&str>>();

                match sections[0].to_lowercase().as_str() {
                    "flashcard" => Card::FlashCard(FlashCard::parse_raw(sections[1].to_string())),
                    "multiple_choice" => {
                        Card::MultipleChoice(MultipleChoice::parse_raw(sections[1].to_string()))
                    }
                    "multiple_answer" => {
                        Card::MultipleAnswer(MultipleAnswer::parse_raw(sections[1].to_string()))
                    }
                    "fill_in_the_blanks" => {
                        Card::FillInTheBlanks(FillInTheBlanks::parse_raw(sections[1].to_string()))
                    }
                    "order" => Card::Order(Order::parse_raw(sections[1].to_string())),
                    // Replace with ParsingError datatype
                    _ => panic!("Parsing Error"),
                }
            })
            .collect();

        Ok(cards)
    }
}
