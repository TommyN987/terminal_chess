use domain::game::GameState;

use crate::ids::PlayerId;

pub(crate) struct Game {
    player_1: PlayerId,
    player_2: PlayerId,
    game_state: GameState,
}
