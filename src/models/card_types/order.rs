use core::fmt;

use rand::seq::SliceRandom;

use crate::{
    models::{choice::Choice, errors::parsing_error::ParsingError, stateful_list::StatefulList},
    Card, UserAnswer,
};

pub struct Order {
    pub question: String,
    pub shuffled: StatefulList<Choice>,
    pub answer: Vec<String>,

    pub user_answer: UserAnswer,
}

impl Order {
    pub fn instructions(&self) -> String {
        return String::from(
            "<SPACE>: Select item, use <Space> again on another item to swap them",
        );
    }

    pub fn validate_answer(&mut self) -> UserAnswer {
        let choices = self
            .shuffled
            .items
            .iter()
            .map(|item| item.content.to_string())
            .collect::<Vec<String>>();

        self.user_answer = if choices == self.answer {
            UserAnswer::Correct
        } else {
            UserAnswer::Incorrect
        };

        self.user_answer
    }

    pub fn parse_raw(content: String) -> Result<Self, ParsingError> {
        let (question, content) = Card::extract_card_title(&content)?;
        let mut rng = rand::thread_rng();

        let mut shuffled: Vec<Choice> = content
            .lines()
            .map(|line| Choice {
                content: line[3..].to_string(),
                selected: false,
            })
            .collect();

        shuffled.shuffle(&mut rng);

        Ok(Self {
            question,
            shuffled: StatefulList::with_items(shuffled),
            answer: content.lines().map(|line| line[3..].to_string()).collect(),
            user_answer: UserAnswer::Undecided,
        })
    }

    /// Check if there are multiple items currently selected
    pub fn multiple_selected(&self) -> Option<(usize, usize)> {
        let selected: Vec<i32> = self
            .shuffled
            .items
            .iter()
            .enumerate()
            .map(|(i, card)| if card.selected { i as i32 } else { -1 })
            .filter(|item| *item >= 0)
            .collect();

        if selected.len() != 2 {
            None
        } else {
            Some((selected[0] as usize, selected[1] as usize))
        }
    }

    /// Unselect all items held within the internal vector
    pub fn unselect_all(&mut self) {
        for choice in self.shuffled.items.iter_mut() {
            choice.unselect();
        }
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Question: {}\nShuffled: {:?}\nAnswer: {:?}",
            self.question, self.shuffled.items, self.answer
        )
    }
}
