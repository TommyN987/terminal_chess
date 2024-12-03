use ratatui::crossterm::event::KeyEvent;

use crate::{
    application::{App, AppResult},
    handlers::commands::CommandRegistry,
};

use super::KeyEventHandler;

pub struct MainMenuHandler {
    registry: CommandRegistry,
}

impl MainMenuHandler {
    pub fn new() -> Self {
        Self {
            registry: CommandRegistry::init_main_menu_registry(),
        }
    }
}

impl KeyEventHandler for MainMenuHandler {
    fn handle_key_event(&self, key_event: KeyEvent, state: &mut App) -> AppResult<()> {
        if let Some(command) = self.registry.get(&key_event).cloned() {
            command.0.execute(state)
        } else {
            Ok(())
        }
    }
}
