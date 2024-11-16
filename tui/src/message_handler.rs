use std::{
    collections::HashMap,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, MouseEvent};

use crate::{
    app::{App, AppResult, Direction, EventContext},
    handlers::strategies::{GameHandler, KeyEventHandler, MainMenuHandler, PromotionMenuHandler},
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Tick,
    Quit,
    KeyPress(KeyEvent),
    MousePress(MouseEvent),
}

pub struct MessageHandler {
    sender: mpsc::Sender<Message>,
    receiver: mpsc::Receiver<Message>,
    handler: thread::JoinHandle<()>,
    key_event_handlers: HashMap<EventContext, Box<dyn KeyEventHandler>>,
}

impl MessageHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("No events available") {
                        match event::read().expect("Unable to read event") {
                            Event::Key(e) => sender.send(Message::KeyPress(e)),
                            Event::Mouse(e) => sender.send(Message::MousePress(e)),
                            _ => unimplemented!(),
                        }
                        .expect("Failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender
                            .send(Message::Tick)
                            .expect("Failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };

        let mut key_event_handlers: HashMap<EventContext, Box<dyn KeyEventHandler>> =
            HashMap::new();

        key_event_handlers.insert(
            EventContext::PromotionMenu,
            Box::new(PromotionMenuHandler::new()),
        );

        key_event_handlers.insert(EventContext::Game, Box::new(GameHandler::new()));

        key_event_handlers.insert(EventContext::MainMenu, Box::new(MainMenuHandler::new()));

        Self {
            sender,
            receiver,
            handler,
            key_event_handlers,
        }
    }

    pub fn next(&self) -> AppResult<Message> {
        Ok(self.receiver.recv()?)
    }

    pub fn handle_key_events(&self, key_event: KeyEvent, app: &mut App) -> AppResult<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }

        if let Some(handler) = self.key_event_handlers.get(&app.event_context) {
            handler.handle_key_event(key_event, app);
        }

        Ok(())
    }

    fn handle_menu_key_events(&self, key_event: KeyEvent, app: &mut App) -> AppResult<()> {
        match key_event.code {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Up | KeyCode::Char('k') => app.move_menu_cursor(Direction::North),
            KeyCode::Down | KeyCode::Char('j') => app.move_menu_cursor(Direction::South),
            KeyCode::Enter => app.run(),
            _ => {}
        }

        Ok(())
    }
}
