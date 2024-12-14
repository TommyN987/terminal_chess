use std::collections::HashMap;

use domain::game::GameState;
use tokio::sync::{mpsc, RwLock};

use crate::{ids::GameId, matchmaker::GameRequest};

#[derive(Default)]
pub struct GlobalState {
    pub active_games: RwLock<HashMap<GameId, GameState>>,
    pub matchmaker: RwLock<Option<mpsc::Sender<GameRequest>>>,
}
