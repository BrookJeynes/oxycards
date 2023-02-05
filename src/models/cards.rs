use super::card::Card;

pub struct Cards {
    pub current_card: usize,
    pub cards: Vec<Card>,
}

impl Cards {
    /// Create a new struct with a series of Cards
    pub fn with_cards(cards: Vec<Card>) -> Self {
        Self {
            current_card: 0,
            cards,
        }
    }

    /// Move the internally selected item forward.
    pub fn next(&mut self) {
        if self.current_card < self.cards.len() - 1 {
            self.current_card += 1;
        }
    }

    /// Move the internally selected item backwards.
    pub fn previous(&mut self) {
        if self.current_card > 0 {
            self.current_card -= 1;
        }
    }

    /// Return the current selected Card
    pub fn selected(&mut self) -> &mut Card {
        self.cards
            .get_mut(self.current_card)
            .expect("Will always return a valid card")
    }
}
