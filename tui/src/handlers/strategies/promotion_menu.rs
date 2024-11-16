use ratatui::crossterm::event::KeyEvent;

use crate::{
    app::{App, AppResult},
    handlers::commands::CommandRegistry,
};

use super::KeyEventHandler;

pub struct PromotionMenuHandler {
    registry: CommandRegistry,
}

impl PromotionMenuHandler {
    pub fn new() -> Self {
        Self {
            registry: CommandRegistry::init_promotion_menu_registry(),
        }
    }
}

impl KeyEventHandler for PromotionMenuHandler {
    fn handle_key_event(&self, key_event: KeyEvent, state: &mut App) -> AppResult<()> {
        if let Some(command) = self.registry.get(&key_event).cloned() {
            command.0.execute(state)
        } else {
            Ok(())
        }
    }
}
