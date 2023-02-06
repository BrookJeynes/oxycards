use tui::widgets::ListState;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    /// Create a StatefulList with the items passed in.
    pub fn with_items(items: Vec<T>) -> Self {
        let mut stateful_list = Self {
            state: ListState::default(),
            items,
        };

        // Auto select first item in decks list
        stateful_list.next();

        stateful_list
    }

    /// Move the internally selected item forward.
    pub fn next(&mut self) {
        if !self.items.is_empty() {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.items.len() - 1 {
                        i
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };

            self.state.select(Some(i))
        }
    }

    /// Move the internally selected item backwards.
    pub fn previous(&mut self) {
        if !self.items.is_empty() {
            let i = match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        i
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };

            self.state.select(Some(i))
        }
    }

    /// Swap two items.
    pub fn swap(&mut self, a: usize, b: usize) {
        self.items.swap(a, b);
    }

    /// Return the selected items index.
    pub fn selected(&self) -> Option<usize> {
        if self.items.is_empty() {
            return None;
        }

        self.state.selected()
    }

    /// Return the selected items value.
    pub fn selected_value(&mut self) -> Option<&mut T> {
        match self.selected() {
            Some(index) => Some(
                self.items
                    .get_mut(index)
                    .expect("Will always return a valid card"),
            ),
            None => None,
        }
    }
}
