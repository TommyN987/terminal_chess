use derive_new::new;
use domain::moves::MoveType;

use crate::app::{App, AppResult, EventContext};

use super::Command;

#[derive(Debug, Clone, new)]
pub(super) struct PromotionMenuNavigationCommand {
    direction: i8,
}

impl Command for PromotionMenuNavigationCommand {
    fn execute(&self, app: &mut App) -> AppResult<()> {
        if let Some(ref mut promotion_menu) = app.game.view_state.promotion_menu {
            let len = promotion_menu.pieces.len();
            let new_index = ((promotion_menu.selected as isize + self.direction as isize)
                + len as isize) as usize
                % len;
            promotion_menu.selected = new_index;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(super) struct PromotionMenuEnterCommand;

impl Command for PromotionMenuEnterCommand {
    fn execute(&self, app: &mut App) -> AppResult<()> {
        if let Some(promotion_menu) = &app.game.view_state.promotion_menu {
            let selected_piece_type = promotion_menu.pieces[promotion_menu.selected]
                .clone()
                .inner()
                .piece_type;
            let mut promotion_move = promotion_menu.m.clone();
            promotion_move.move_type = MoveType::Promotion(selected_piece_type.into());

            app.game.game_state.make_move(promotion_move);
            app.game.view_state.promotion_menu = None;
            app.game.view_state.currently_legal_moves.clear();
            match app.game.game_state.is_game_over() {
                true => app.event_context = EventContext::GameOver,
                false => app.event_context = EventContext::Game,
            }
        }

        Ok(())
    }
}
