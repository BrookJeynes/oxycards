use core::fmt;

use crate::{extract_card_title, Choice};
use rand::prelude::SliceRandom;

use super::stateful_list::StatefulList;

pub struct Order {
    pub question: String,
    pub shuffled: StatefulList<Choice>,
    pub answer: Vec<String>,
    pub correct_answer: Option<bool>,
}

impl Order {
    pub fn parse_raw(content: String) -> Self {
        let (question, content) = extract_card_title(&content);
        let mut rng = rand::thread_rng();

        let mut shuffled: Vec<Choice> = content
            .lines()
            .map(|line| Choice {
                content: line[3..].to_string(),
                selected: false,
            })
            .collect();

        // Todo: shuffle until shuffled != answer
        shuffled.shuffle(&mut rng);

        Self {
            question,
            shuffled: StatefulList::with_items(shuffled),
            answer: content.lines().map(|line| line[3..].to_string()).collect(),
            correct_answer: None,
        }
    }

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

    pub fn unselect_all(&mut self) {
        for choice in self.shuffled.items.iter_mut() {
            choice.unselect();
        }
    }

    pub fn validate_answer(&mut self) -> Option<bool> {
        let choices = self
            .shuffled
            .items
            .iter()
            .map(|item| item.content.to_string())
            .collect::<Vec<String>>();

        if self.answer == choices {
            self.correct_answer = Some(true);
        } else {
            self.correct_answer = Some(false);
        }

        self.correct_answer
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
