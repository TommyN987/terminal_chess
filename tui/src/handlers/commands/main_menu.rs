use derive_new::new;
use domain::board::Direction;

use crate::application::{App, AppResult, GameType};

use super::Command;

#[derive(Debug, Clone, new)]
pub(super) struct MainMenuNavigationCommand {
    direction: Direction,
}

impl Command for MainMenuNavigationCommand {
    fn execute(&self, state: &mut App) -> AppResult<()> {
        match self.direction {
            Direction::North => state.menu.next(),
            Direction::South => state.menu.previous(),
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(super) struct MainMenuEnterCommand;

impl Command for MainMenuEnterCommand {
    fn execute(&self, state: &mut App) -> AppResult<()> {
        state.start_game(GameType::default());
        Ok(())
    }
}
