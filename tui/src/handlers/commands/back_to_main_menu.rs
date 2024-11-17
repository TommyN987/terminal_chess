use crate::app::{App, AppResult, CurrentScreen, EventContext};

use super::Command;

#[derive(Debug, Clone)]
pub(super) struct BackToMainMenuCommand;

impl Command for BackToMainMenuCommand {
    fn execute(&self, app: &mut App) -> AppResult<()> {
        app.current_screen = CurrentScreen::Menu;
        app.event_context = EventContext::MainMenu;
        Ok(())
    }
}
