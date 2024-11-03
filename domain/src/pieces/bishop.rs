use crate::{board::Board, direction::Direction, position::Position};

use super::moveable::{Move, MoveType, Moveable};

#[derive(Debug, Clone)]
pub struct Bishop {
    directions: Vec<Direction>,
}

impl Default for Bishop {
    fn default() -> Self {
        Self::new()
    }
}

impl Bishop {
    pub fn new() -> Self {
        Self {
            directions: vec![
                Direction::NorthEast,
                Direction::NorthWest,
                Direction::SouthEast,
                Direction::SouthWest,
            ],
        }
    }
}

impl Moveable for Bishop {
    fn get_moves(&self, from: Position, board: &Board) -> Vec<Move> {
        self.reachable_positions_in_many_directions(from, board, &self.directions)
            .into_iter()
            .map(|pos| Move::new(MoveType::Normal, from, pos))
            .collect()
    }
}
