pub mod game;
pub mod game_over;
pub mod main_menu;
pub mod promotion_menu;

pub use game::*;
pub use game_over::*;
pub use main_menu::*;
pub use promotion_menu::*;

use ratatui::crossterm::event::KeyEvent;

use crate::app::{App, AppResult};

pub trait KeyEventHandler {
    fn handle_key_event(&self, key_event: KeyEvent, state: &mut App) -> AppResult<()>;
}
