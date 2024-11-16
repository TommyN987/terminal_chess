use app::{App, AppResult};
use message_handler::{Message, MessageHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;

mod app;
mod constants;
mod game;
mod handlers;
mod menu;
mod message_handler;
mod tui;
mod widgets;

fn main() -> AppResult<()> {
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let message_handler = MessageHandler::new(250);
    let mut tui = Tui::new(terminal, message_handler);
    let mut app = App::default();

    tui.init()?;

    while app.is_running {
        tui.draw(&mut app)?;

        match tui.message_handler.next()? {
            Message::Tick => app.tick(),
            Message::KeyPress(key_event) => {
                tui.message_handler.handle_key_events(key_event, &mut app)?
            }
            _ => {}
        }
    }

    tui.exit()?;

    Ok(())
}
