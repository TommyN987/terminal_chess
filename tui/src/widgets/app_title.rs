use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{Block, Paragraph, Widget},
};

use crate::constants::TITLE;

pub struct AppTitle;

impl Widget for AppTitle {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let lines: Vec<&str> = TITLE.trim().lines().collect();
        let text_height = lines.len() as u16;
        let vertical_offset = area.y + (area.height.saturating_sub(text_height)) / 2;

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
