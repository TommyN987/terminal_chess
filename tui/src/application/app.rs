use domain::game::GameState;
use ratatui::{layout::Rect, Frame};

use super::{Game, GameType, Menu};

pub type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct App {
    pub is_running: bool,
    pub current_screen: CurrentScreen,
    pub event_context: EventContext,
    pub menu: Menu,
    pub game: Option<Game>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            is_running: true,
            current_screen: CurrentScreen::MainMenu,
            event_context: EventContext::MainMenu,
            menu: Menu::default(),
            game: None,
        }
    }
}

impl App {
    pub fn start_game(&mut self, game_type: GameType) {
        let new_game = GameState::default();
        self.game = Some(Game::new(new_game, game_type));
        self.current_screen = CurrentScreen::Game;
        self.event_context = EventContext::Game;
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }

    pub fn tick(&self) {}

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        match self.current_screen {
            CurrentScreen::MainMenu => self.menu.render_self(frame, area),
            CurrentScreen::Game => {
                if let Some(game) = &self.game {
                    game.render_self(frame, area);
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum CurrentScreen {
    MainMenu,
    Game,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EventContext {
    MainMenu,
    Game,
    PromotionMenu,
    GameOver,
}
