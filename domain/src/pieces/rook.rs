use crate::{board::Board, direction::Direction, position::Position};

use super::moveable::{Move, MoveType, Moveable};

#[derive(Debug, Clone)]
pub struct Rook {
    directions: Vec<Direction>,
}

impl Default for Rook {
    fn default() -> Self {
        Self::new()
    }
}

impl Rook {
    pub fn new() -> Self {
        Self {
            directions: vec![
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::West,
            ],
        }
    }
}

impl Moveable for Rook {
    fn get_moves(&self, from: Position, board: &Board) -> Vec<Move> {
        self.reachable_positions_in_many_directions(from, board, &self.directions)
            .into_iter()
            .map(|pos| Move::new(MoveType::Normal, from, pos))
            .collect()
    }
}
