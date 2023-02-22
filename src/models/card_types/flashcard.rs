use core::fmt;

use crate::{Card, UserAnswer};

pub struct FlashCard {
    pub question: String,
    pub answer: String,
    pub flipped: bool,
    pub show_validation_popup: bool,
    /// Has the card been validated/answered
    pub user_answer: UserAnswer,
}

impl FlashCard {
    pub fn instructions(&self) -> String {
        String::from("<SPACE>: Show cards back")
    }

    pub fn validate_answer(&mut self) -> UserAnswer {
        self.show_validation_popup = !self.show_validation_popup;

        UserAnswer::Undecided
    }

    pub fn parse_raw(content: String) -> Self {
        let (question, content) = Card::extract_card_title(&content);

        Self {
            question,
            answer: content,
            flipped: false,

            show_validation_popup: false,
            user_answer: UserAnswer::Undecided,
        }
    }

    /// Flip card over to show the back.
    pub fn show_back(&mut self) {
        self.flipped = true;
    }

    /// Flip the card
    pub fn flip_card(&mut self) {
        self.flipped = !self.flipped;
    }
}

impl fmt::Display for FlashCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Question: {}\nAnswer: {}", self.question, self.answer)
    }
}
