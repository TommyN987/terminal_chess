use derive_new::new;

use crate::{board::Board, player::Player};

#[derive(Debug, Clone, PartialEq, new)]
pub struct Game {
    pub board: Board,
    pub current_player: Player,
}
