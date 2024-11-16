use domain::pieces::{Move, MoveType};

use crate::{
    app::{App, AppResult, Direction, EventContext},
    widgets::promotion_menu::PromotionMenu,
};

use super::Command;

#[derive(Debug, Clone)]
pub(super) struct BoardNavigationCommand(Direction);

impl BoardNavigationCommand {
    pub(super) fn new(direction: Direction) -> Self {
        Self(direction)
    }
}

impl Command for BoardNavigationCommand {
    fn execute(&self, app: &mut App) -> AppResult<()> {
        if app.game.view_state.promotion_menu.is_none() {
            app.game.move_cursor(self.0.clone());
        }
        Ok(())
    }
}

pub(super) struct BoardEnterCommand;

impl Command for BoardEnterCommand {
    fn execute(&self, app: &mut App) -> AppResult<()> {
        match app.game.view_state.selected_position {
            None => app.game.select_piece(),
            Some(_) => self.handle_piece_or_move(app)?,
        }

        Ok(())
    }
}

impl BoardEnterCommand {
    fn handle_piece_or_move(&self, app: &mut App) -> AppResult<()> {
        let cursor_position = app.game.view_state.cursor_position;

        if let Some(piece) = app.game.game_state.board.get(&cursor_position) {
            if app.game.game_state.current_player.color == piece.piece_color {
                app.game.select_piece();
                return Ok(());
            }
        }

        if let Some(m) = self.find_move_to_cursor(app) {
            if m.move_type == MoveType::Promotion {
                self.open_promotion_menu(app, m)?;
                app.event_context = EventContext::PromotionMenu;
            } else {
                app.game.game_state.make_move(m);
                app.game.view_state.currently_legal_moves.clear();
            }
        }

        Ok(())
    }

    fn find_move_to_cursor(&self, app: &App) -> Option<Move> {
        let cursor_position = app.game.view_state.cursor_position;
        app.game
            .view_state
            .currently_legal_moves
            .iter()
            .find(|m| m.to == cursor_position)
            .copied()
    }

    fn open_promotion_menu(&self, app: &mut App, promotion_move: Move) -> AppResult<()> {
        app.game.view_state.promotion_menu = Some(PromotionMenu::new(
            app.game.game_state.current_player.color,
            promotion_move,
        ));
        Ok(())
    }
}
