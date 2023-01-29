use core::fmt;

use crate::extract_card_title;

pub struct MultipleAnswer {
    pub question: String,
    pub choices: Vec<String>,
    pub answers: Vec<String>,
}

impl MultipleAnswer {
    pub fn parse_raw(content: String) -> Self {
        let (question, content) = extract_card_title(&content);

        Self {
            question,
            choices: MultipleAnswer::remove_prefix(' ', &content),
            answers: MultipleAnswer::remove_prefix('*', &content),
        }
    }

    fn remove_prefix(prefix: char, content: &String) -> Vec<String> {
        content
            .lines()
            .filter(|item| item.chars().nth(1).unwrap() == prefix)
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
            self.question, self.choices, self.answers
        )
    }
}
