use crate::application::{App, AppResult};

use super::Command;

pub(super) struct QuitCommand;

impl Command for QuitCommand {
    fn execute(&self, app: &mut App) -> AppResult<()> {
        app.quit();
        Ok(())
    }
}
