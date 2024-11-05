use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::widgets::menu::AppTitle;

pub const MENU_ITEMS: [&str; 4] = ["Human vs. Human", "Human vs. Engine", "Online Game", "Help"];

#[derive(Debug, Clone, Copy)]
pub struct Menu {
    pub items: [&'static str; 4],
    pub selected: usize,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            items: MENU_ITEMS,
            selected: 0,
        }
    }
}

impl Menu {
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

    pub fn render_self(mut self, frame: &mut Frame, main_area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(main_area);

        frame.render_widget(AppTitle, layout[0]);

        frame.render_stateful_widget(self.clone(), layout[1], &mut self.selected);
    }
}
