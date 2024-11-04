use ratatui::Frame;

use crate::game::{CurrentScreen, Game};

use super::{board::render_game_state, menu::render_menu};

pub fn render(game: &mut Game, frame: &mut Frame) {
    let main_area = frame.area();

    match game.current_screen {
        CurrentScreen::Menu => render_menu(frame, main_area, game),
        CurrentScreen::Game => {
            if let Some(ref mut game_state) = game.game_state.clone() {
                render_game_state(frame, main_area, game_state);
            }
        }
        CurrentScreen::Exit => {}
    }
}
