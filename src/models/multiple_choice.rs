use core::fmt;

use crate::{extract_card_title, Choice};

use super::stateful_list::StatefulList;

pub struct MultipleChoice {
    pub question: String,
    pub choices: StatefulList<Choice>,
    pub answers: Vec<String>,
    pub correct_answer: Option<bool>,
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
            correct_answer: None,
        }
    }

    fn remove_prefix(prefix: Vec<char>, content: &String) -> Vec<String> {
        content
            .lines()
            // Todo: Don't unwrap
            .filter(|item| prefix.contains(&item.chars().nth(0).unwrap()))
            .map(|item| item[1..].trim().to_string())
            .collect()
    }

    pub fn unselect_all(&mut self) {
        for choice in self.choices.items.iter_mut() {
            choice.unselect();
        }
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

impl fmt::Display for MultipleChoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Question: {}\nChoices: {:?}\nAnswers: {:?}",
            self.question, self.choices.items, self.answers
        )
    }
}
