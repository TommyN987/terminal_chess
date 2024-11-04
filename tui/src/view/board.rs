use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, StatefulWidget, Widget},
    Frame,
};

use domain::{board::Board as DomainBoard, game::GameState};

use super::{
    constants::{BLACK, WHITE},
    piece::Piece,
};

pub fn render_game_state(frame: &mut Frame, main_area: Rect, game_state: &mut GameState) {
    let main_layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 18),
                Constraint::Ratio(16, 18),
                Constraint::Ratio(1, 18),
            ]
            .as_ref(),
        )
        .split(main_area);

    let main_layout_vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(2, 17),
                Constraint::Ratio(9, 17),
                Constraint::Ratio(1, 17),
                Constraint::Ratio(5, 17),
            ]
            .as_ref(),
        )
        .split(main_layout_horizontal[1]);

    frame.render_stateful_widget(
        Board::from(game_state.board.clone()),
        main_layout_vertical[1],
        &mut Board::from(game_state.board.clone()),
    );
}

pub struct Board(DomainBoard);

impl From<DomainBoard> for Board {
    fn from(value: DomainBoard) -> Self {
        Self(value)
    }
}

impl Board {
    fn inner(&mut self) -> &mut DomainBoard {
        &mut self.0
    }
}

impl StatefulWidget for Board {
    type State = Board;

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

        state
            .inner()
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
                    let cell_color: Color = if (i + j) % 2 == 0 { WHITE } else { BLACK };

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
