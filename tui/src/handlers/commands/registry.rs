use std::collections::HashMap;

use domain::board::Direction;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

use super::{
    BackToMainMenuCommand, BoardEnterCommand, BoardNavigationCommand, Command, CommandBox,
    MainMenuEnterCommand, MainMenuNavigationCommand, PromotionMenuEnterCommand,
    PromotionMenuNavigationCommand, QuitCommand,
};

#[derive(Clone)]
pub struct CommandRegistry {
    commands: HashMap<KeyEvent, CommandBox>,
}

impl CommandRegistry {
    pub fn init_main_menu_registry() -> Self {
        let mut registry = Self::new();

        registry.register(
            KeyEvent::from(KeyCode::Down),
            MainMenuNavigationCommand::new(Direction::South),
        );

        registry.register(
            KeyEvent::from(KeyCode::Char('j')),
            MainMenuNavigationCommand::new(Direction::South),
        );

        registry.register(
            KeyEvent::from(KeyCode::Up),
            MainMenuNavigationCommand::new(Direction::North),
        );

        registry.register(
            KeyEvent::from(KeyCode::Char('k')),
            MainMenuNavigationCommand::new(Direction::North),
        );

        registry.register(KeyEvent::from(KeyCode::Enter), MainMenuEnterCommand);

        registry.register(KeyEvent::from(KeyCode::Char('q')), QuitCommand);

        registry
    }

    pub fn init_promotion_menu_registry() -> Self {
        let mut registry = Self::new();

        registry.register(
            KeyEvent::from(KeyCode::Left),
            PromotionMenuNavigationCommand::new(Direction::West),
        );
        registry.register(
            KeyEvent::from(KeyCode::Right),
            PromotionMenuNavigationCommand::new(Direction::East),
        );

        registry.register(KeyEvent::from(KeyCode::Enter), PromotionMenuEnterCommand);

        registry
    }

    pub fn init_game_over_registry() -> Self {
        let mut registry = Self::new();

        registry.register(KeyEvent::from(KeyCode::Enter), BackToMainMenuCommand);

        registry
    }

    pub fn init_game_registry() -> Self {
        let mut registry = Self::new();

        registry.register(
            KeyEvent::from(KeyCode::Up),
            BoardNavigationCommand::new(Direction::North),
        );
        registry.register(
            KeyEvent::from(KeyCode::Char('k')),
            BoardNavigationCommand::new(Direction::North),
        );
        registry.register(
            KeyEvent::from(KeyCode::Down),
            BoardNavigationCommand::new(Direction::South),
        );
        registry.register(
            KeyEvent::from(KeyCode::Char('j')),
            BoardNavigationCommand::new(Direction::South),
        );
        registry.register(
            KeyEvent::from(KeyCode::Left),
            BoardNavigationCommand::new(Direction::West),
        );
        registry.register(
            KeyEvent::from(KeyCode::Char('h')),
            BoardNavigationCommand::new(Direction::West),
        );
        registry.register(
            KeyEvent::from(KeyCode::Right),
            BoardNavigationCommand::new(Direction::East),
        );
        registry.register(
            KeyEvent::from(KeyCode::Char('l')),
            BoardNavigationCommand::new(Direction::East),
        );
        registry.register(KeyEvent::from(KeyCode::Enter), BoardEnterCommand);

        registry.register(KeyEvent::from(KeyCode::Char('q')), QuitCommand);

        registry
    }
}

impl CommandRegistry {
    fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn get(&self, key_event: &KeyEvent) -> Option<&CommandBox> {
        self.commands.get(key_event)
    }

    fn register<T: Command + Send + Sync + 'static>(&mut self, key_event: KeyEvent, command: T) {
        self.commands.insert(key_event, CommandBox::new(command));
    }
}
