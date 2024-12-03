use ratatui::crossterm::event::KeyEvent;

use crate::{
    application::{App, AppResult},
    handlers::commands::CommandRegistry,
};

use super::KeyEventHandler;

pub struct GameOverHandler {
    registry: CommandRegistry,
}

impl GameOverHandler {
    pub fn new() -> Self {
        Self {
            registry: CommandRegistry::init_game_over_registry(),
        }
    }
}

impl KeyEventHandler for GameOverHandler {
    fn handle_key_event(&self, key_event: KeyEvent, state: &mut App) -> AppResult<()> {
        if let Some(command) = self.registry.get(&key_event).cloned() {
            command.0.execute(state)
        } else {
            Ok(())
        }
    }
}
