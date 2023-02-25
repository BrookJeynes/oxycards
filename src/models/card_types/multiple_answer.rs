use core::fmt;

use crate::{
    models::{choice::Choice, errors::parsing_error::ParsingError, stateful_list::StatefulList},
    Card, UserAnswer,
};

pub struct MultipleAnswer {
    pub question: String,
    pub choices: StatefulList<Choice>,
    pub answers: Vec<String>,
    pub user_answer: UserAnswer,
}

impl MultipleAnswer {
    pub fn instructions(&self) -> String {
        String::from("<SPACE>: Select/unselect choice")
    }

    pub fn validate_answer(&mut self) -> UserAnswer {
        let choices = self
            .choices
            .items
            .iter()
            .filter(|item| item.selected)
            .map(|item| item.content.to_string())
            .collect::<Vec<String>>();

        self.user_answer = if choices.is_empty() {
            UserAnswer::Undecided
        } else if choices == self.answers {
            UserAnswer::Correct
        } else {
            UserAnswer::Incorrect
        };

        self.user_answer
    }

    pub fn parse_raw(content: String) -> Result<Self, ParsingError> {
        let (question, content) = Card::extract_card_title(&content)?;

        Ok(Self {
            question,
            choices: StatefulList::with_items(
                MultipleAnswer::remove_prefix(vec![' ', '*'], &content)
                    .iter()
                    .map(|choice| Choice {
                        content: choice.clone(),
                        selected: false,
                    })
                    .collect(),
            ),
            answers: MultipleAnswer::remove_prefix(vec!['*'], &content),
            user_answer: UserAnswer::Undecided,
        })
    }

    /// Remove prefix (* | -) from item
    fn remove_prefix(prefix: Vec<char>, content: &String) -> Vec<String> {
        content
            .lines()
            .filter(|item| prefix.contains(&item.chars().nth(1).unwrap()))
            .map(|item| item[3..].trim().to_string())
            .collect()
    }
}

impl fmt::Display for MultipleAnswer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Question: {}\nChoices: {:?}\nAnswers: {:?}",
            self.question, self.choices.items, self.answers
        )
    }
}
