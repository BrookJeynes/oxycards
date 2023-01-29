use core::fmt;

use crate::extract_card_title;

use super::stateful_list::StatefulList;

pub struct MultipleChoice {
    pub question: String,
    pub choices: StatefulList<String>,
    pub answers: Vec<String>,
}

impl MultipleChoice {
    pub fn parse_raw(content: String) -> Self {
        let (question, content) = extract_card_title(&content);

        Self {
            question,
            choices: StatefulList::with_items(MultipleChoice::remove_prefix(
                vec!['-', '*'],
                &content,
            )),
            answers: MultipleChoice::remove_prefix(vec!['*'], &content),
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
