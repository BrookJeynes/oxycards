#[derive(Debug)]
pub struct Choice {
    pub content: String,
    pub selected: bool,
}

impl Choice {
    /// Flip the current selected status
    pub fn select(&mut self) {
        self.selected = !self.selected;
    }

    /// Unselect the current card
    pub fn unselect(&mut self) {
        self.selected = false;
    }
}
