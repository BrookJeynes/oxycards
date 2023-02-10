use core::fmt;
use regex::Regex;

use crate::{extract_card_title, models::user_answer::UserAnswer};

#[derive(Debug)]
pub struct Answer {
    pub answers: Vec<String>,
    pub content: String,
}

pub struct FillInTheBlanks {
    pub question: String,
    pub content: String,
    pub user_input: Vec<String>,
    pub current_input: Vec<String>,
    pub answers: Vec<String>,
    pub blank_index: usize,
    pub user_answer: UserAnswer,
}

impl FillInTheBlanks {
    pub fn parse_raw(content: String) -> Self {
        let (question, content) = extract_card_title(&content);
        let re = Regex::new(r"_(.*?)_").expect("Error with regex string.");

        let answers: Vec<String> = re
            .captures_iter(content.as_ref())
            .flat_map(|c| {
                let capture = c[1].to_string();

                let capture: Vec<&str> = capture.split("|").collect();

                // Bug: For every multiple choice fill-in-the-blank, there will be an additional
                // spot the user has to scoll.
                capture
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
            })
            .collect();

        // Create an array with empty string of size answers
        let user_input: Vec<String> = answers.iter().map(|_| String::new()).collect();
        let current_input: Vec<String> = answers.iter().map(|_| String::from("")).collect();

        Self {
            question,
            content: re.replace_all(content.as_ref(), "__").to_string(),
            answers,
            user_input,
            current_input,
            blank_index: 0,
            user_answer: UserAnswer::Undecided,
        }
    }

    /// Move to the next fill-in-the-blank spot
    pub fn next(&mut self) {
        self.blank_index = (self.blank_index + 1) % self.answers.len();
    }

    pub fn instructions(&self) -> String {
        // TODO add instructions
        return String::from("ESC: Quit, TAB: Next blank spot");
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

        // Bug: The user doesn't have to get the answers in the correct order, but much rather have
        // the correct answers written down
        for item in self.user_input.iter() {
            if !self.answers.contains(item) && !item.is_empty() {
                self.user_answer = UserAnswer::Incorrect;
            }
        }

        self.user_answer
    }

    pub fn check_answered(&self) -> bool {
        self.user_answer != UserAnswer::Undecided
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
