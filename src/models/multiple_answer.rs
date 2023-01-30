use core::fmt;

use crate::{extract_card_title, Choice};

use super::stateful_list::StatefulList;

pub struct MultipleAnswer {
    pub question: String,
    pub choices: StatefulList<Choice>,
    pub answers: Vec<String>,
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
        }
    }

    fn remove_prefix(prefix: Vec<char>, content: &String) -> Vec<String> {
        content
            .lines()
            .filter(|item| prefix.contains(&item.chars().nth(1).unwrap()))
            // Todo: Don't unwrap
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
