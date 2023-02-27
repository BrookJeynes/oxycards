use core::fmt;

use crossterm::style::Stylize;

use crate::{reset_terminal, UserAnswer};

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
                    eprintln!(
                        "{}: {}",
                        "Parsing Error".red().bold(),
                        ParsingError::IncorrectDivider
                    );
                    reset_terminal().unwrap();
                    std::process::exit(1);
                }

                match sections[0].to_lowercase().as_str() {
                    "flashcard" => Card::FlashCard(
                        FlashCard::parse_raw(sections[1].to_string()).unwrap_or_else(|err| {
                            eprintln!("{}: {}", "Parsing Error".red().bold(), err);
                            reset_terminal().unwrap();
                            std::process::exit(1);
                        }),
                    ),
                    "multiple_choice" => Card::MultipleChoice(
                        MultipleChoice::parse_raw(sections[1].to_string()).unwrap_or_else(|err| {
                            eprintln!("{}: {}", "Parsing Error".red().bold(), err);
                            reset_terminal().unwrap();
                            std::process::exit(1);
                        }),
                    ),
                    "multiple_answer" => Card::MultipleAnswer(
                        MultipleAnswer::parse_raw(sections[1].to_string()).unwrap_or_else(|err| {
                            eprintln!("{}: {}", "Parsing Error".red().bold(), err);
                            reset_terminal().unwrap();
                            std::process::exit(1);
                        }),
                    ),
                    "fill_in_the_blanks" => Card::FillInTheBlanks(
                        FillInTheBlanks::parse_raw(sections[1].to_string()).unwrap_or_else(|err| {
                            eprintln!("{}: {}", "Parsing Error".red().bold(), err);
                            reset_terminal().unwrap();
                            std::process::exit(1);
                        }),
                    ),
                    "order" => Card::Order(
                        Order::parse_raw(sections[1].to_string()).unwrap_or_else(|err| {
                            eprintln!("{}: {}", "Parsing Error".red().bold(), err);
                            reset_terminal().unwrap();
                            std::process::exit(1);
                        }),
                    ),
                    _ => {
                        eprintln!(
                            "{}: {}",
                            "Parsing Error".red().bold(),
                            ParsingError::NoCardType
                        );
                        reset_terminal().unwrap();
                        std::process::exit(1);
                    }
                }
            })
            .collect();

        Ok(cards)
    }
}
