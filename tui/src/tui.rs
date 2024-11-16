use ratatui::{
    backend::Backend,
    crossterm::{
        self,
        event::{DisableMouseCapture, EnableMouseCapture},
        terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use std::{
    io,
    panic::{set_hook, take_hook},
};

use crate::{
    app::{App, AppResult},
    message_handler::MessageHandler,
};

pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
    pub message_handler: MessageHandler,
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>, message_handler: MessageHandler) -> Self {
        Self {
            terminal,
            message_handler,
        }
    }

    pub fn init(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        let panic_hook = take_hook();
        set_hook(Box::new(move |panic| {
            Self::reset().expect("Failed to reset terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        self.terminal.draw(|frame| {
            let main_area = frame.area();
            app.render(frame, main_area);
        })?;
        Ok(())
    }

    pub fn exit(&mut self) -> AppResult<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    fn reset() -> AppResult<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }
}
