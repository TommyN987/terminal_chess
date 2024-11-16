use derive_new::new;

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

            app.game.game_state.promotion_move = Some((promotion_menu.m, selected_piece_type));
            app.game.game_state.make_move(promotion_menu.m);
            app.game.view_state.promotion_menu = None;
            app.event_context = EventContext::Game;
            app.game.view_state.currently_legal_moves.clear();
        }

        Ok(())
    }
}
