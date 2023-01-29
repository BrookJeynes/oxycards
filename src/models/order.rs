use core::fmt;

use crate::extract_card_title;
use rand::prelude::SliceRandom;

pub struct Order {
    pub question: String,
    pub shuffled: Vec<String>,
    pub answer: Vec<String>,
}

impl Order {
    pub fn parse_raw(content: String) -> Self {
        let (question, content) = extract_card_title(&content);
        let mut rng = rand::thread_rng();

        let mut shuffled: Vec<String> = content.lines().map(|line| line[3..].to_string()).collect();

        // Todo: shuffle until shuffled != answer
        shuffled.shuffle(&mut rng);

        Self {
            question,
            shuffled,
            answer: content.lines().map(|line| line[3..].to_string()).collect(),
        }
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Question: {}\nShuffled: {:?}\nAnswer: {:?}",
            self.question, self.shuffled, self.answer
        )
    }
}
