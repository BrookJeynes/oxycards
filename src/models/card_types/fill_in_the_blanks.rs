use core::fmt;
use regex::Regex;
use std::collections::HashMap;

use crate::{
    models::{card::Card, errors::parsing_error::ParsingError},
    UserAnswer,
};

#[derive(Debug)]
pub struct Answer {
    pub answers: Vec<String>,
    pub content: String,
}

pub struct FillInTheBlanks {
    pub question: String,
    pub content: String,
    pub output: String,
    pub user_input: Vec<String>,
    pub answers: HashMap<usize, Vec<String>>,
    pub blank_index: usize,
    pub user_answer: UserAnswer,
}

impl FillInTheBlanks {
    pub fn check_answered(&self) -> bool {
        false
    }

    pub fn parse_raw(content: String) -> Result<Self, ParsingError> {
        let (question, content) = Card::extract_card_title(&content)?;
        let re = Regex::new(r"_(.*?)_").expect("Error with regex string.");

        let answers = HashMap::from(
            re.captures_iter(content.as_ref())
                .enumerate()
                .map(|(index, c)| {
                    let capture: Vec<String> =
                        c[1].split("|").map(|item| item.to_string()).collect();

                    (index, capture)
                })
                .collect::<HashMap<usize, Vec<String>>>(),
        );

        // Create an array with empty string of size answers
        let user_input: Vec<String> = answers.iter().map(|_| String::new()).collect();

        Ok(Self {
            question,
            content: re.replace_all(content.as_ref(), "__").to_string(),
            answers,
            output: re.replace_all(content.as_ref(), "").to_string(),
            user_input,
            blank_index: 0,
            user_answer: UserAnswer::Undecided,
        })
    }

    /// Move to the next fill-in-the-blank spot
    pub fn next(&mut self) {
        self.blank_index = (self.blank_index + 1) % self.answers.len();
    }

    pub fn instructions(&self) -> String {
        String::from("<ESC>: Quit application, <TAB>: Cycle selection, <Char>: Add character pressed to blank space")
    }

    pub fn validate_answer(&mut self) -> UserAnswer {
        self.user_answer = UserAnswer::Correct;

        if self
            .user_input
            .iter()
            .filter(|item| !item.is_empty())
            .collect::<Vec<&String>>()
            .is_empty()
        {
            self.user_answer = UserAnswer::Undecided;
        }

        for (index, item) in self.user_input.iter().enumerate() {
            if !self.answers.get(&index).unwrap_or(&vec![]).contains(item) && !item.is_empty() {
                self.user_answer = UserAnswer::Incorrect;
            }
        }

        self.user_answer
    }

    pub fn update_output(&mut self) {
        let new_content = self
            .content
            .split("__")
            .take(self.answers.len())
            .enumerate()
            .map(|(index, item)| {
                format!(
                    "{}{}",
                    item,
                    self.user_input.get(index).unwrap_or(&String::new())
                )
            })
            .collect::<Vec<String>>()
            .join("");

        self.output = new_content;
    }
}

impl fmt::Display for FillInTheBlanks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Question: {}\nContent: {:?}\nAnswers: {:?}",
            self.question, self.content, self.answers
        )
    }
}
