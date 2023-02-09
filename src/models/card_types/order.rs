use core::fmt;

use rand::seq::SliceRandom;

use crate::{
    extract_card_title,
    models::{choice::Choice, stateful_list::StatefulList},
    UserAnswer,
};

pub struct Order {
    pub question: String,
    pub shuffled: StatefulList<Choice>,
    pub answer: Vec<String>,

    pub user_answer: UserAnswer,
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
            user_answer: UserAnswer::Undecided,
        }
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

    pub fn instructions(&self) -> String {
        return String::from("SPACE: Select first item, press SPACE again on another item to swap");
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

    pub fn check_answered(&self) -> bool {
        self.user_answer != UserAnswer::Undecided
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
