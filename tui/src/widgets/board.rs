use domain::board::{Board as DomainBoard, Position};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, Widget},
};

use crate::{
    application::ViewState,
    constants::{BLACK, WHITE},
};

use super::piece::Piece;

pub struct Board<'a>(&'a DomainBoard, &'a ViewState);

impl<'a> Board<'a> {
    pub fn new(board: &'a DomainBoard, view_state: &'a ViewState) -> Self {
        Self(board, view_state)
    }

    pub fn inner(&self) -> (&DomainBoard, &ViewState) {
        (self.0, self.1)
    }
}

impl<'a> Widget for Board<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let (board, view_state) = self.inner();
        let width = area.width / 8;
        let height = area.height / 8;
        let border_height = area.height / 2 - (4 * height);
        let border_width = area.width / 2 - (4 * width);

        let columns = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(border_height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(height),
                    Constraint::Length(border_height),
                ]
                .as_ref(),
            )
            .split(area);

        board.fields.iter().enumerate().for_each(|(i, row)| {
            let lines = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(border_width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(width),
                        Constraint::Length(border_width),
                    ]
                    .as_ref(),
                )
                .split(columns[i + 1]);
            row.iter().enumerate().for_each(|(j, c)| {
                let mut cell_color: Color = if (i + j) % 2 == 0 { WHITE } else { BLACK };

                if let Some(selected) = view_state.selected_position {
                    if selected == Position::from((i, j)) {
                        cell_color = Color::Cyan;
                    }
                }

                view_state.currently_legal_moves.iter().for_each(|pos| {
                    if pos.to == Position::from((i, j)) {
                        cell_color = Color::Magenta;
                    }
                });

                if view_state.cursor_position == Position::from((i, j)) {
                    cell_color = Color::Blue;
                }

                let cell = Block::default().bg(cell_color);
                let square = lines[j + 1];
                Widget::render(cell, square, buf);
                if let Some(piece) = c {
                    Piece::from(piece).render(square, buf);
                }
            })
        });
    }
}
