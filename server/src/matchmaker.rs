use std::collections::VecDeque;

use tokio::sync::mpsc;

use crate::ids::PlayerId;

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

    pub async fn run(&mut self) {
        while let Some(request) = self.rx.recv().await {
            println!("Received game request: {:?}", request);

            if let Some(opponent) = self.pending.pop_front() {
                println!("Matched {:?} with {:?}", opponent, request);
                self.start_game(opponent, request).await;
            } else {
                self.pending.push_back(request);
            }
        }
    }

    async fn start_game(&self, player_1: GameRequest, player_2: GameRequest) {
        println!(
            "Starting game between {:?} and {:?}",
            player_1.player_id, player_2.player_id
        );
        // TODO: Implement game initialization and state management
    }
}
