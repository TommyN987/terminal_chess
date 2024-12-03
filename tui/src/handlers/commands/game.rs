use domain::{
    board::Direction,
    moves::{Move, MoveType},
};

use crate::{
    application::{App, AppResult, EventContext, Game},
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
        app.game.as_mut().map(|game| game.move_cursor(self.0));
        Ok(())
    }
}

pub(super) struct BoardEnterCommand;

impl Command for BoardEnterCommand {
    fn execute(&self, app: &mut App) -> AppResult<()> {
        if let Some(game) = app.game.as_mut() {
            match game.view_state.selected_position {
                None => game.select_piece(),
                Some(_) => self.handle_piece_or_move(app)?,
            }
        }

        Ok(())
    }
}

impl BoardEnterCommand {
    fn handle_piece_or_move(&self, app: &mut App) -> AppResult<()> {
        if let Some(game) = app.game.as_mut() {
            let cursor_position = game.view_state.cursor_position;

            if let Some(piece) = game.game_state.board[&cursor_position] {
                if game.game_state.current_player.color == piece.piece_color {
                    game.select_piece();
                    return Ok(());
                }
            }

            if let Some(m) = self.find_move_to_cursor(game) {
                if matches!(m.move_type, MoveType::Promotion(_)) {
                    self.open_promotion_menu(app, m)?;
                    app.event_context = EventContext::PromotionMenu;
                } else {
                    game.game_state.make_move(m);
                    game.view_state.currently_legal_moves.clear();
                    if game.game_state.is_game_over() {
                        app.event_context = EventContext::GameOver;
                    }
                }
            }
        }

        Ok(())
    }

    fn find_move_to_cursor(&self, game: &Game) -> Option<Move> {
        let cursor_position = game.view_state.cursor_position;
        game.view_state
            .currently_legal_moves
            .iter()
            .find(|m| m.to == cursor_position)
            .cloned()
    }

    fn open_promotion_menu(&self, app: &mut App, promotion_move: Move) -> AppResult<()> {
        app.game.as_mut().map(|game| {
            game.promotion_menu = Some(PromotionMenu::new(
                game.game_state.current_player.color,
                promotion_move,
            ))
        });
        Ok(())
    }
}
