use core::fmt;

use crate::extract_card_title;

pub struct FlashCard {
    pub question: String,
    pub answer: String,
    pub flipped: bool,
}

impl FlashCard {
    pub fn parse_raw(content: String) -> Self {
        let (question, content) = extract_card_title(&content);

        Self {
            question,
            answer: content,
            flipped: false,
        }
    }

    pub fn flip_card(&mut self) {
        self.flipped = !self.flipped;
    }
}

impl fmt::Display for FlashCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Question: {}\nAnswer: {}", self.question, self.answer)
    }
}
