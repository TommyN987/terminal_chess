use game::{AppResult, Game};
use message_handler::{Message, MessageHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use view::tui::Tui;

mod game;
mod menu;
mod message_handler;
mod view;

fn main() -> AppResult<()> {
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let message_handler = MessageHandler::new(250);
    let mut tui = Tui::new(terminal, message_handler);
    let mut game = Game::default();

    tui.init()?;

    while game.is_running {
        tui.draw(&mut game)?;

        match tui.message_handler.next()? {
            Message::Tick => game.tick(),
            Message::KeyPress(key_event) => tui
                .message_handler
                .handle_key_events(key_event, &mut game)?,
            _ => {}
        }
    }

    tui.exit()?;

    Ok(())
}
