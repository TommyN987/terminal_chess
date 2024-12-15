use std::{collections::VecDeque, sync::Arc};

use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    game_session::GameSession,
    global_state::GlobalState,
    ids::{GameId, PlayerId},
};

#[derive(Debug)]
pub struct GameRequest {
    pub player_id: PlayerId,
}

pub struct Matchmaker {
    rx: mpsc::Receiver<GameRequest>,
    pending: VecDeque<GameRequest>,
}

impl Matchmaker {
    pub fn new(rx: mpsc::Receiver<GameRequest>) -> Self {
        Self {
            rx,
            pending: VecDeque::new(),
        }
    }

    pub async fn run(&mut self, global_state: Arc<GlobalState>) {
        while let Some(request) = self.rx.recv().await {
            println!("Received game request: {:?}", request);

            if let Some(opponent) = self.pending.pop_front() {
                println!("Matched {:?} with {:?}", opponent, request);
                self.start_game(opponent, request, Arc::clone(&global_state))
                    .await;
            } else {
                self.pending.push_back(request);
            }
        }
    }

    async fn start_game(
        &self,
        player_1: GameRequest,
        player_2: GameRequest,
        global_state: Arc<GlobalState>,
    ) {
        let game_id = GameId::from(Uuid::new_v4());

        println!(
            "Starting game between {:?} and {:?} with game ID: {:?}",
            player_1.player_id, player_2.player_id, game_id
        );

        let session = GameSession::new(player_1.player_id, player_2.player_id);

        global_state
            .active_games
            .write()
            .await
            .insert(game_id, session);
    }
}
