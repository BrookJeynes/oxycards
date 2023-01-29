use core::fmt;
use regex::Regex;

use crate::extract_card_title;

pub struct FillInTheBlanks {
    pub question: String,
    pub content: String,
    pub answers: Vec<String>,
}

impl FillInTheBlanks {
    pub fn parse_raw(content: String) -> Self {
        let (question, content) = extract_card_title(&content);
        let re = Regex::new(r"_(.*?)_").expect("Error with regex string.");

        Self {
            question,
            content: re.replace_all(content.as_ref(), "__").to_string(),
            answers: re
                .captures_iter(content.as_ref())
                .map(|capture| capture[1].to_string())
                .collect(),
        }
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
