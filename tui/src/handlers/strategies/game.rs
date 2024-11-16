use ratatui::crossterm::event::KeyEvent;

use crate::{
    app::{App, AppResult},
    handlers::commands::CommandRegistry,
};

use super::KeyEventHandler;

pub struct GameHandler {
    pub registry: CommandRegistry,
}

impl GameHandler {
    pub fn new() -> Self {
        Self {
            registry: CommandRegistry::init_game_registry(),
        }
    }
}

impl KeyEventHandler for GameHandler {
    fn handle_key_event(&self, key_event: KeyEvent, state: &mut App) -> AppResult<()> {
        if let Some(command) = self.registry.get(&key_event).cloned() {
            command.0.execute(state)
        } else {
            Ok(())
        }
    }
}
