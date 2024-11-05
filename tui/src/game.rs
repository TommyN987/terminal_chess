use domain::{game::GameState, position::Position};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::app::Direction as CursorDirection;

#[derive(Debug, Clone)]
pub struct Game {
    pub game_state: GameState,
    pub view_state: ViewState,
    pub is_running: bool,
}

impl Game {
    pub fn new(game_state: GameState) -> Self {
        Self {
            game_state,
            view_state: ViewState::default(),
            is_running: true,
        }
    }

    pub fn move_cursor(&mut self, direction: CursorDirection) {
        self.view_state.cursor_position += direction.into();

        if self.view_state.cursor_position.row > 7 {
            self.view_state.cursor_position.row = 0;
        }

        if self.view_state.cursor_position.row < 0 {
            self.view_state.cursor_position.row = 7;
        }

        if self.view_state.cursor_position.column > 7 {
            self.view_state.cursor_position.column = 0;
        }

        if self.view_state.cursor_position.column < 0 {
            self.view_state.cursor_position.column = 7;
        }
    }

    pub fn select_piece(&mut self) {
        if let Some(_) = self.game_state.board.get(&self.view_state.cursor_position) {
            let position = self.view_state.cursor_position;
            self.view_state.selected_position = Some(position.clone());
            if let Some((_, moves)) = self.game_state.legal_moves_for_piece(position) {
                self.view_state.currently_legal_moves = moves.into_iter().map(|m| m.to).collect();
            }
        }
    }

    pub fn exit(&mut self) {
        self.is_running = false;
    }

    pub fn render_self(mut self, frame: &mut Frame, main_area: Rect) {
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

        frame.render_stateful_widget(self.clone(), main_layout_vertical[1], &mut self.view_state);
    }
}

#[derive(Debug, Clone)]
pub struct ViewState {
    pub cursor_position: Position,
    pub selected_position: Option<Position>,
    pub currently_legal_moves: Vec<Position>,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            cursor_position: Position { row: 6, column: 3 },
            selected_position: None,
            currently_legal_moves: vec![],
        }
    }
}
