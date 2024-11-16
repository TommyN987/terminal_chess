use derive_new::new;

use crate::app::{App, AppResult, Direction};

use super::Command;

#[derive(Debug, Clone, new)]
pub(super) struct MainMenuNavigationCommand {
    direction: Direction,
}

impl Command for MainMenuNavigationCommand {
    fn execute(&self, state: &mut App) -> AppResult<()> {
        state.move_menu_cursor(self.direction.clone());
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(super) struct MainMenuEnterCommand;

impl Command for MainMenuEnterCommand {
    fn execute(&self, state: &mut App) -> AppResult<()> {
        state.run();
        Ok(())
    }
}
