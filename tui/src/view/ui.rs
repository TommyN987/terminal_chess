use ratatui::Frame;

use crate::game::{CurrentScreen, Game};

use super::menu::render_menu;

pub fn render(game: &mut Game, frame: &mut Frame) {
    let main_area = frame.size();

    match game.current_screen {
        CurrentScreen::Menu => render_menu(frame, main_area, game),
        CurrentScreen::Game => {}
        CurrentScreen::Exit => {}
    }
}
