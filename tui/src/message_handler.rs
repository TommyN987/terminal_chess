use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use domain::pieces::{MoveType, PieceType};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, MouseEvent};

use crate::{
    app::{App, AppResult, CurrentScreen, Direction},
    widgets::promotion_menu::PromotionMenu,
};

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

    pub fn handle_key_events(&self, key_event: KeyEvent, app: &mut App) -> AppResult<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }

        match app.current_screen {
            CurrentScreen::Menu => self.handle_menu_key_events(key_event, app),
            CurrentScreen::Game => self.handle_game_key_events(key_event, app),
            _ => Ok(()),
        }
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

    fn handle_game_key_events(&self, key_event: KeyEvent, app: &mut App) -> AppResult<()> {
        match key_event.code {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Up | KeyCode::Char('k') => {
                if app.game.view_state.promotion_menu.is_none() {
                    app.game.move_cursor(Direction::North)
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.game.view_state.promotion_menu.is_none() {
                    app.game.move_cursor(Direction::South)
                }
            }
            KeyCode::Left | KeyCode::Char('h') => match app.game.view_state.promotion_menu {
                None => app.game.move_cursor(Direction::West),
                Some(ref mut promotion_menu) => {
                    if promotion_menu.selected <= 0 {
                        promotion_menu.selected = 3;
                    } else {
                        promotion_menu.selected -= 1;
                    }
                }
            },
            KeyCode::Right | KeyCode::Char('l') => match app.game.view_state.promotion_menu {
                None => app.game.move_cursor(Direction::East),
                Some(ref mut promotion_menu) => {
                    if promotion_menu.selected >= 3 {
                        promotion_menu.selected = 0;
                    } else {
                        promotion_menu.selected += 1;
                    }
                }
            },
            KeyCode::Enter => match app.game.view_state.selected_position {
                None => app.game.select_piece(),
                Some(_) => match &app.game.view_state.promotion_menu {
                    Some(promotion_menu) => {
                        app.game.game_state.promotion_move = Some((
                            promotion_menu.m,
                            promotion_menu.pieces[promotion_menu.selected]
                                .clone()
                                .inner()
                                .piece_type,
                        ));
                        app.game.game_state.make_move(promotion_menu.m);
                        app.game.view_state.promotion_menu = None;
                        app.game.view_state.currently_legal_moves.clear();
                    }
                    None => {
                        let cursor_position = app.game.view_state.cursor_position;

                        match app.game.game_state.board.get(&cursor_position) {
                            None => {
                                let maybe_move = app
                                    .game
                                    .view_state
                                    .currently_legal_moves
                                    .iter()
                                    .find(|m| m.to == cursor_position);

                                if let Some(m) = maybe_move {
                                    if m.move_type == MoveType::Promotion {
                                        app.game.view_state.promotion_menu =
                                            Some(PromotionMenu::new(
                                                app.game.game_state.current_player.color,
                                                *m,
                                            ));
                                    } else {
                                        app.game.game_state.make_move(*m);
                                        app.game.view_state.currently_legal_moves.clear();
                                    }
                                }
                            }
                            Some(piece) => {
                                if app.game.game_state.current_player.color == piece.piece_color {
                                    app.game.select_piece();
                                    return Ok(());
                                }
                                let maybe_move = app
                                    .game
                                    .view_state
                                    .currently_legal_moves
                                    .iter()
                                    .find(|m| m.to == cursor_position);

                                if let Some(m) = maybe_move {
                                    if m.move_type == MoveType::Promotion {
                                        app.game.view_state.promotion_menu =
                                            Some(PromotionMenu::new(
                                                app.game.game_state.current_player.color,
                                                *m,
                                            ));
                                    } else {
                                        app.game.game_state.make_move(*m);
                                        app.game.view_state.currently_legal_moves.clear();
                                    }
                                }
                            }
                        }
                    }
                },
            },
            _ => {}
        };
        Ok(())
    }
}
