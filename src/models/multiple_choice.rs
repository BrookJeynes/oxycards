use core::fmt;

use crate::extract_card_title;

pub struct MultipleChoice {
    pub question: String,
    pub choices: Vec<String>,
    pub answers: Vec<String>,
}

impl MultipleChoice {
    pub fn parse_raw(content: String) -> Self {
        let (question, content) = extract_card_title(&content);

        Self {
            question,
            choices: MultipleChoice::remove_prefix('-', &content),
            answers: MultipleChoice::remove_prefix('*', &content),
        }
    }

    fn remove_prefix(prefix: char, content: &String) -> Vec<String> {
        content
            .lines()
            // Todo: Don't unwrap
            .filter(|item| item.chars().nth(0).unwrap() == prefix)
            .map(|item| item[1..].trim().to_string())
            .collect()
    }
}

impl fmt::Display for MultipleChoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Question: {}\nChoices: {:?}\nAnswers: {:?}",
            self.question, self.choices, self.answers
        )
    }
}
