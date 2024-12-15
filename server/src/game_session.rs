use domain::game::GameState;

use crate::ids::PlayerId;

pub struct GameSession {
    pub player_1: PlayerId,
    pub player_2: PlayerId,
    pub game_state: GameState,
}

impl GameSession {
    pub fn new(player_1: PlayerId, player_2: PlayerId) -> Self {
        Self {
            player_1,
            player_2,
            game_state: GameState::new(),
        }
    }

    pub fn contains_player(&self, player_id: &PlayerId) -> bool {
        &self.player_1 == player_id || &self.player_2 == player_id
    }
}
