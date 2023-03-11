use crate::Errors;
use core::fmt;

use crate::UserAnswer;

use super::{
    card_types::{
        fill_in_the_blanks::FillInTheBlanks, flashcard::FlashCard, multiple_answer::MultipleAnswer,
        multiple_choice::MultipleChoice, order::Order,
    },
    errors::parsing_error::ParsingError,
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

macro_rules! parse_cards {
    ($(($card_variant:ident, $card_type:expr)),*) => {
        impl Card {
            pub fn card_parser(content: String) -> Result<Vec<Self>, ParsingError> {
                let cards: Vec<Card> = content
                    .split("---")
                    .map(|section| {
                        let sections = section
                            .trim()
                            .split("\n\n")
                            .filter(|item| !item.is_empty())
                            .collect::<Vec<&str>>();

                        if sections.is_empty() {
                            Errors::throw_parsing_error(ParsingError::IncorrectDivider)
                        }

                        match sections[0].to_lowercase().as_str() {
                            $($card_type => Card::$card_variant(
                                $card_variant::parse_raw(sections[1].to_string()).unwrap_or_else(
                                    |err| {
                                        Errors::throw_parsing_error(err)
                                    },
                                ),
                            )),*,
                            _ => {
                                Errors::throw_parsing_error(ParsingError::NoCardType)
                            }
                        }
                    })
                    .collect();

                Ok(cards)
            }
        }
    };
}

parse_cards!(
    (FlashCard, "flashcard"),
    (MultipleAnswer, "multiple_answer"),
    (MultipleChoice, "multiple_choice"),
    (FillInTheBlanks, "fill_in_the_blanks"),
    (Order, "order")
);

impl Card {
    pub fn extract_card_title(content: &String) -> Result<(String, String), ParsingError> {
        let question = match content.lines().nth(0) {
            Some(val) => {
                if val.is_empty() {
                    return Err(ParsingError::NoQuestion);
                }

                if val.chars().nth(0).unwrap_or(' ') != '#' {
                    return Err(ParsingError::NoQuestion);
                }

                val[1..].trim().to_string()
            }
            None => return Err(ParsingError::NoQuestion),
        };

        let content = content.lines().skip(1).collect::<Vec<&str>>().join("\n");

        if content.is_empty() {
            return Err(ParsingError::NoContent);
        }

        Ok((question, content))
    }
}
