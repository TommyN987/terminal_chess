pub const MENU_ITEMS: [&str; 4] = ["Human vs. Human", "Human vs. Engine", "Online Game", "Help"];

#[derive(Debug, Clone)]
pub struct MenuState {
    pub items: [&'static str; 4],
    pub selected: usize,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            items: MENU_ITEMS,
            selected: 0,
        }
    }
}

impl MenuState {
    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.items.len() - 1;
        }
    }
}
