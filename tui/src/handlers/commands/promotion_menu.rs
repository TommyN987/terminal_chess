use derive_new::new;
use domain::{board::Direction, moves::MoveType};

use crate::application::{App, AppResult, EventContext};

use super::Command;

#[derive(Debug, Clone, new)]
pub(super) struct PromotionMenuNavigationCommand {
    direction: Direction,
}

impl Command for PromotionMenuNavigationCommand {
    fn execute(&self, app: &mut App) -> AppResult<()> {
        app.game.as_mut().map(|game| {
            game.promotion_menu.as_mut().map(|pm| {
                match self.direction {
                    Direction::East => pm.next(),
                    Direction::West => pm.previous(),
                    _ => {}
                };
            })
        });
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(super) struct PromotionMenuEnterCommand;

impl Command for PromotionMenuEnterCommand {
    fn execute(&self, app: &mut App) -> AppResult<()> {
        app.game.as_mut().map(|game| {
            game.promotion_menu.as_mut().map(|pm| {
                let selected_piece_type = pm.pieces[pm.selected].clone().inner().piece_type;
                let mut promotion_move = pm.m.clone();
                promotion_move.move_type = MoveType::Promotion(selected_piece_type.into());
                game.game_state.make_move(promotion_move);
            });
            game.promotion_menu = None;
            game.view_state.currently_legal_moves.clear();
            match game.game_state.is_game_over() {
                true => app.event_context = EventContext::GameOver,
                false => app.event_context = EventContext::Game,
            }
        });

        Ok(())
    }
}
