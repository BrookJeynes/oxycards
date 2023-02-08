use core::fmt;

use crate::{
    extract_card_title,
    models::{choice::Choice, stateful_list::StatefulList},
};

pub struct MultipleAnswer {
    pub question: String,
    pub choices: StatefulList<Choice>,
    pub answers: Vec<String>,
    pub answered: bool,

    pub correct_answer: Option<bool>,
}

impl MultipleAnswer {
    pub fn parse_raw(content: String) -> Self {
        let (question, content) = extract_card_title(&content);

        Self {
            question,
            choices: StatefulList::with_items(
                MultipleAnswer::remove_prefix(vec![' ', '*'], &content)
                    .iter()
                    .map(|choice| Choice {
                        // Todo: maybe don't clone?
                        content: choice.clone(),
                        selected: false,
                    })
                    .collect(),
            ),
            answers: MultipleAnswer::remove_prefix(vec!['*'], &content),
            answered: false,
            correct_answer: None,
        }
    }

    /// Remove prefix (* | -) from item
    fn remove_prefix(prefix: Vec<char>, content: &String) -> Vec<String> {
        content
            .lines()
            .filter(|item| prefix.contains(&item.chars().nth(1).unwrap()))
            .map(|item| item[3..].trim().to_string())
            .collect()
    }

    pub fn instructions() -> String {
        return String::from("SPACE: Select/unselect choice");
    }

    pub fn validate_answer(&mut self) -> Option<bool> {
        let choices = self
            .choices
            .items
            .iter()
            .filter(|item| item.selected)
            .map(|item| item.content.to_string())
            .collect::<Vec<String>>();

        if choices.is_empty() {
            self.correct_answer = None;
        } else if self.answers == choices {
            self.correct_answer = Some(true);
        } else {
            self.correct_answer = Some(false);
        }

        self.correct_answer
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
