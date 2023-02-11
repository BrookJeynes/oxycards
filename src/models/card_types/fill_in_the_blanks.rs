use core::fmt;
use regex::Regex;

use crate::extract_card_title;

use crate::UserAnswer;

#[derive(Debug)]
pub struct Answer {
    pub answers: Vec<String>,
    pub content: String,
}

pub struct FillInTheBlanks {
    pub question: String,
    pub content: String,
    pub answers: Vec<Answer>,
    pub blank_index: usize,
    pub user_answer: UserAnswer
}

impl FillInTheBlanks {
    pub fn instructions(&self) -> String {
        // TODO add instructions
        return String::from("");
    }

    pub fn validate_answer(&mut self) -> UserAnswer {
        UserAnswer::Undecided
    }

    pub fn check_answered(&self) -> bool {
        false
    }

    pub fn parse_raw(content: String) -> Self {
        let (question, content) = extract_card_title(&content);
        let re = Regex::new(r"_(.*?)_").expect("Error with regex string.");

        Self {
            question,
            content: re.replace_all(content.as_ref(), "__").to_string(),
            answers: re
                .captures_iter(content.as_ref())
                .map(|c| {
                    let capture = c[1].to_string();

                    Answer {
                        answers: capture
                            .split("|")
                            .map(|answer| answer.to_string())
                            .collect(),
                        content: String::new(),
                    }
                })
                .collect(),
            blank_index: 0,
            user_answer: UserAnswer::Undecided
        }
    }

    /// Move to the next fill-in-the-blank spot
    fn next(&mut self) {
        self.blank_index = self.blank_index % self.answers.len();
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
