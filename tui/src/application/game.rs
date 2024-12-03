use domain::{
    board::{Direction, Position},
    game::{Color, GameState},
    moves::{Move, MoveType},
    pieces::PromotionPiece,
};
use ratatui::{
    layout::{Constraint, Direction as LayoutDirection, Layout, Rect},
    Frame,
};

use crate::widgets::{promotion_menu::PromotionMenu, Board};

#[derive(Debug, Clone)]
pub struct Game {
    pub game_state: GameState,
    pub view_state: ViewState,
    pub game_type: GameType,
    pub promotion_menu: Option<PromotionMenu>,
}

impl Game {
    pub fn new(game_state: GameState, game_type: GameType) -> Self {
        Self {
            game_state,
            view_state: ViewState::default(),
            game_type,
            promotion_menu: None,
        }
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        self.view_state.cursor_position += direction;

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
        if let Some(_) = self.game_state.board[&self.view_state.cursor_position] {
            let position = self.view_state.cursor_position;
            self.view_state.selected_position = Some(position.clone());
            self.view_state.currently_legal_moves.clear();
            self.view_state.currently_legal_moves.extend(
                self.game_state
                    .legal_moves_for_piece(position)
                    .map_or(vec![], |(_, moves)| moves),
            );
        }
    }

    pub fn render_self(&self, frame: &mut Frame, area: Rect) {
        let main_layout = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints(
                [
                    Constraint::Ratio(1, 18),
                    Constraint::Ratio(16, 18),
                    Constraint::Ratio(1, 18),
                ]
                .as_ref(),
            )
            .split(area);

        let layout_vertical = Layout::default()
            .direction(LayoutDirection::Horizontal)
            .constraints(
                [
                    Constraint::Ratio(2, 17),
                    Constraint::Ratio(9, 17),
                    Constraint::Ratio(1, 17),
                    Constraint::Ratio(5, 17),
                ]
                .as_ref(),
            )
            .split(main_layout[1]);

        let history_area = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints(
                [
                    Constraint::Ratio(2, 15),
                    Constraint::Ratio(11, 15),
                    Constraint::Ratio(2, 15),
                ]
                .as_ref(),
            )
            .split(layout_vertical[3]);

        let board = Board::new(&self.game_state.board, &self.view_state);

        frame.render_widget(board, layout_vertical[1]);
        if let Some(promotion_menu) = &self.promotion_menu {
            frame.render_widget(promotion_menu.clone(), layout_vertical[1]);
        }
    }
}

#[derive(Debug, Clone)]
pub struct ViewState {
    pub cursor_position: Position,
    pub selected_position: Option<Position>,
    pub currently_legal_moves: Vec<Move>,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            cursor_position: Position::from((6, 3)),
            selected_position: None,
            currently_legal_moves: Vec::with_capacity(17), // 17 is the maximum potential number of moves for a queen
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum GameType {
    #[default]
    Normal,
    Online,
    AgainstBot,
}
