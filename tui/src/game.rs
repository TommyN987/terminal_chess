use domain::game::GameState;

use crate::menu::MenuState;

pub type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

pub enum CurrentScreen {
    Menu,
    Game,
    Exit,
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Game {
    pub is_running: bool,
    pub current_screen: CurrentScreen,
    pub menu_state: MenuState,
    pub game_state: Option<GameState>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            is_running: true,
            current_screen: CurrentScreen::Menu,
            menu_state: MenuState::default(),
            game_state: None,
        }
    }
}

impl Game {
    pub fn run(&mut self) {
        self.current_screen = CurrentScreen::Game;
    }

    pub fn move_menu_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.menu_state.previous(),
            Direction::South => self.menu_state.next(),
            _ => {}
        }
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }

    pub fn tick(&self) {}
}
