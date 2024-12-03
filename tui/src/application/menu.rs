use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, Padding},
    Frame,
};

use crate::widgets::AppTitle;

pub const MENU_ITEMS: [&str; 4] = ["Human vs. Human", "Human vs. Engine", "Online Game", "Help"];

#[derive(Debug, Clone)]
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

    pub fn render_self(&self, frame: &mut Frame, main_area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(main_area);

        let menu_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50)])
            .flex(Flex::Center)
            .split(layout[1]);

        let list_items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, &item)| {
                if self.selected == i {
                    ListItem::new(Text::from(format!("> {}", item)).alignment(Alignment::Center))
                        .style(
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )
                } else {
                    ListItem::new(Text::from(item).alignment(Alignment::Center))
                        .style(Style::default())
                }
            })
            .collect();

        let list = List::new(list_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .padding(Padding::top(2))
                    .title("Options")
                    .title_alignment(Alignment::Center),
            )
            .style(Style::default().fg(Color::White));

        frame.render_widget(AppTitle, layout[0]);
        frame.render_widget(list, menu_layout[0]);
    }
}
