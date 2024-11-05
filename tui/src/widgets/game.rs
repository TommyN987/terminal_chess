use domain::position::Position;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, StatefulWidget, Widget},
};

use crate::{
    constants::{BLACK, WHITE},
    game::{Game, ViewState},
};

use super::piece::Piece;

impl StatefulWidget for Game {
    type State = ViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let cell_side_length = area.width / 8;
        let border_length = area.width / 2 - (4 * cell_side_length);

        let columns = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(border_length),
                    Constraint::Length(cell_side_length),
                    Constraint::Length(cell_side_length),
                    Constraint::Length(cell_side_length),
                    Constraint::Length(cell_side_length),
                    Constraint::Length(cell_side_length),
                    Constraint::Length(cell_side_length),
                    Constraint::Length(cell_side_length),
                    Constraint::Length(cell_side_length),
                    Constraint::Length(border_length),
                ]
                .as_ref(),
            )
            .split(area);

        self.game_state
            .board
            .fields
            .iter()
            .enumerate()
            .for_each(|(i, row)| {
                let lines = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Length(border_length),
                            Constraint::Length(cell_side_length),
                            Constraint::Length(cell_side_length),
                            Constraint::Length(cell_side_length),
                            Constraint::Length(cell_side_length),
                            Constraint::Length(cell_side_length),
                            Constraint::Length(cell_side_length),
                            Constraint::Length(cell_side_length),
                            Constraint::Length(cell_side_length),
                            Constraint::Length(border_length),
                        ]
                        .as_ref(),
                    )
                    .split(columns[i + 1]);
                row.iter().enumerate().for_each(|(j, c)| {
                    let mut cell_color: Color = if (i + j) % 2 == 0 { WHITE } else { BLACK };

                    if let Some(selected) = state.selected_position {
                        if selected == Position::from((i, j)) {
                            cell_color = Color::Cyan;
                        }
                    }

                    state.currently_legal_moves.iter().for_each(|pos| {
                        if *pos == Position::from((i, j)) {
                            cell_color = Color::Magenta;
                        }
                    });

                    if state.cursor_position == Position::from((i, j)) {
                        cell_color = Color::Blue;
                    }

                    let cell = Block::default().bg(cell_color);
                    let square = lines[j + 1];
                    Widget::render(cell, square, buf);
                    if let Some(piece) = c {
                        Piece::from(piece).render(square, buf);
                    }
                })
            })
    }
}
