use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, StatefulWidget, Widget},
};

use crate::{constants::TITLE, menu::Menu};

pub struct AppTitle;

impl Widget for AppTitle {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let lines: Vec<&str> = TITLE.trim().lines().collect();
        let text_height = lines.len() as u16;
        let vertical_offset = area.y + (area.height.saturating_sub(text_height)) / 2;

        // Create a new Rect with the vertical offset
        let centered_area = Rect {
            x: area.x,
            y: vertical_offset - 1,
            width: area.width,
            height: text_height + 1,
        };

        Paragraph::new(TITLE)
            .alignment(Alignment::Center)
            .block(Block::default())
            .render(centered_area, buf);
    }
}

impl StatefulWidget for Menu {
    type State = usize;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50)])
            .flex(Flex::Center)
            .split(area);
        let list_items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, &item)| {
                if *state == i {
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

        Widget::render(list, layout[0], buf);
    }
}
