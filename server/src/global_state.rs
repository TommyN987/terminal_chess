use std::collections::HashMap;
use tokio::{
    net::tcp::OwnedWriteHalf,
    sync::{mpsc, RwLock},
};

use crate::{
    game_session::GameSession,
    ids::{GameId, PlayerId},
    matchmaker::GameRequest,
};

#[derive(Default)]
pub struct GlobalState {
    pub active_games: RwLock<HashMap<GameId, GameSession>>,
    pub active_connections: RwLock<HashMap<PlayerId, OwnedWriteHalf>>,
    pub matchmaker: RwLock<Option<mpsc::Sender<GameRequest>>>,
}
