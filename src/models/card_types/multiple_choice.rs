use core::fmt;

use crate::{
    extract_card_title,
    models::{choice::Choice, stateful_list::StatefulList},
    UserAnswer,
};

pub struct MultipleChoice {
    pub question: String,
    pub choices: StatefulList<Choice>,
    pub answers: Vec<String>,

    pub user_answer: UserAnswer,
}

impl MultipleChoice {
    pub fn parse_raw(content: String) -> Self {
        let (question, content) = extract_card_title(&content);

        Self {
            question,
            choices: StatefulList::with_items(
                MultipleChoice::remove_prefix(vec!['-', '*'], &content)
                    .iter()
                    .map(|choice| Choice {
                        // Todo: maybe don't clone?
                        content: choice.clone(),
                        selected: false,
                    })
                    .collect(),
            ),
            answers: MultipleChoice::remove_prefix(vec!['*'], &content),
            user_answer: UserAnswer::Undecided,
        }
    }

    /// Remove prefix (* | -) from item
    fn remove_prefix(prefix: Vec<char>, content: &String) -> Vec<String> {
        content
            .lines()
            // Todo: Don't unwrap
            .filter(|item| prefix.contains(&item.chars().nth(0).unwrap()))
            .map(|item| item[1..].trim().to_string())
            .collect()
    }

    /// Unselect all items held within the internal vector
    pub fn unselect_all(&mut self) {
        for choice in self.choices.items.iter_mut() {
            choice.unselect();
        }
    }

    /// Validate the users current answer
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

    pub fn instructions() -> String {
        return String::from("SPACE: Select choice, ENTER: Validate answer");
    }

    pub fn check_answered(&self) -> bool {
        self.user_answer != UserAnswer::Undecided
    }
}

impl fmt::Display for MultipleChoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Question: {}\nChoices: {:?}\nAnswers: {:?}",
            self.question, self.choices.items, self.answers
        )
    }
}
