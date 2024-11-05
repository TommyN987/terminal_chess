use domain::{direction::Direction as DomainDirection, game::GameState};
use ratatui::{layout::Rect, Frame};

use crate::{game::Game, menu::Menu};

pub struct App {
    pub is_running: bool,
    pub current_screen: CurrentScreen,
    pub menu: Menu,
    pub game: Game,
}

impl Default for App {
    fn default() -> Self {
        Self {
            is_running: true,
            current_screen: CurrentScreen::Menu,
            menu: Menu::default(),
            game: Game::new(GameState::new()),
        }
    }
}

impl App {
    pub fn run(&mut self) {
        self.current_screen = CurrentScreen::Game;
        while self.game.is_running == false {
            self.current_screen = CurrentScreen::Menu;
        }
    }

    pub fn render(&mut self, frame: &mut Frame, main_area: Rect) {
        match self.current_screen {
            CurrentScreen::Menu => self.menu.render_self(frame, main_area),
            CurrentScreen::Game => self.game.clone().render_self(frame, main_area),
            _ => {}
        }
    }

    pub fn move_menu_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.menu.previous(),
            Direction::South => self.menu.next(),
            _ => {}
        }
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }

    pub fn tick(&self) {}
}

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

impl From<DomainDirection> for Direction {
    fn from(value: DomainDirection) -> Self {
        match value {
            DomainDirection::North => Direction::North,
            DomainDirection::South => Direction::South,
            DomainDirection::East => Direction::East,
            DomainDirection::West => Direction::West,
            _ => Direction::South,
        }
    }
}

impl From<Direction> for DomainDirection {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => DomainDirection::North,
            Direction::South => DomainDirection::South,
            Direction::East => DomainDirection::East,
            Direction::West => DomainDirection::West,
        }
    }
}
