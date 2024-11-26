use std::fmt::Display;

use domain::{board::Position, game::GameState, moves::Move};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::{
    app::Direction as CursorDirection,
    widgets::{game::Debugger, promotion_menu::PromotionMenu},
};

#[derive(Clone)]
pub struct Game {
    pub game_state: GameState,
    pub view_state: ViewState,
    pub is_running: bool,
    pub debugger: Vec<String>,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Game state:")?;
        writeln!(f, "Current player: {:?}", self.game_state.current_player)?;
        writeln!(f, "Cursor position: {:?}", self.view_state.cursor_position)?;
        if let Some(pos) = self.view_state.selected_position {
            writeln!(f, "Selected piece: {:?}", self.game_state.board.get(&pos))?;
        }
        writeln!(
            f,
            "Currently legal moves: {:?}",
            self.view_state.currently_legal_moves
        )?;
        for stmt in self.debugger.iter() {
            writeln!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

impl Game {
    pub fn new(game_state: GameState) -> Self {
        Self {
            game_state,
            view_state: ViewState::default(),
            is_running: true,
            debugger: vec![],
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
            if let Some((_, moves)) = self
                .game_state
                .legal_moves_for_piece(self.view_state.selected_position.unwrap())
            {
                self.view_state.currently_legal_moves = moves;
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
        if let Some(ref mut promotion_menu) = self.view_state.promotion_menu {
            frame.render_stateful_widget(
                promotion_menu.clone(),
                main_layout_vertical[1],
                &mut promotion_menu.selected,
            );
        }
        frame.render_stateful_widget(Debugger, main_layout_vertical[3], &mut self);
    }
}

#[derive(Debug, Clone)]
pub struct ViewState {
    pub cursor_position: Position,
    pub selected_position: Option<Position>,
    pub currently_legal_moves: Vec<Move>,
    pub promotion_menu: Option<PromotionMenu>,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            cursor_position: Position { row: 6, column: 3 },
            selected_position: None,
            currently_legal_moves: vec![],
            promotion_menu: None,
        }
    }
}
