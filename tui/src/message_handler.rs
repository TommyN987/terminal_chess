use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use domain::game::GameState;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, MouseEvent};

use crate::game::{AppResult, CurrentScreen, Game};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Tick,
    Quit,
    KeyPress(KeyEvent),
    MousePress(MouseEvent),
}

#[derive(Debug)]
pub struct MessageHandler {
    sender: mpsc::Sender<Message>,
    receiver: mpsc::Receiver<Message>,
    handler: thread::JoinHandle<()>,
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

        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub fn next(&self) -> AppResult<Message> {
        Ok(self.receiver.recv()?)
    }

    pub fn handle_key_events(&self, key_event: KeyEvent, game: &mut Game) -> AppResult<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }

        match game.current_screen {
            CurrentScreen::Menu => self.handle_menu_key_events(key_event, game),
            CurrentScreen::Game => self.handle_game_key_events(key_event, game),
            _ => Ok(()),
        }
    }

    fn handle_menu_key_events(&self, key_event: KeyEvent, game: &mut Game) -> AppResult<()> {
        match key_event.code {
            KeyCode::Char('q') => game.quit(),
            KeyCode::Up | KeyCode::Char('k') => game.menu_state.previous(),
            KeyCode::Down | KeyCode::Char('j') => game.menu_state.next(),
            KeyCode::Enter => {
                game.game_state = Some(GameState::new());
                game.current_screen = CurrentScreen::Game;
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_game_key_events(&self, key_event: KeyEvent, game: &mut Game) -> AppResult<()> {
        match key_event.code {
            KeyCode::Char('q') => game.quit(),
            _ => {}
        };
        Ok(())
    }
}
