use crate::{board::Board, direction::Direction, position::Position};

use super::moveable::{Move, MoveType, Moveable};

pub struct Queen {
    directions: Vec<Direction>,
}

impl Default for Queen {
    fn default() -> Self {
        Self::new()
    }
}

impl Queen {
    pub fn new() -> Self {
        Self {
            directions: vec![
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::West,
                Direction::NorthEast,
                Direction::NorthWest,
                Direction::SouthEast,
                Direction::SouthWest,
            ],
        }
    }
}

impl Moveable for Queen {
    fn get_moves(&self, from: Position, board: &Board) -> Vec<Move> {
        self.reachable_positions_in_many_directions(from, board, &self.directions)
            .into_iter()
            .map(|pos| Move::new(MoveType::Normal, from, pos))
            .collect()
    }
}
