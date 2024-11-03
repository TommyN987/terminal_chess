use crate::{board::Board, direction::Direction, position::Position, Color};

use super::moveable::{Move, MoveType, Moveable};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Queen {
    directions: [Direction; 8],
}

impl Default for Queen {
    fn default() -> Self {
        Self::new()
    }
}

impl Queen {
    pub fn new() -> Self {
        Self {
            directions: [
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
    fn get_moves(
        &self,
        _color: Color,
        _has_moved: bool,
        from: Position,
        board: &Board,
    ) -> Vec<Move> {
        self.reachable_positions_in_many_directions(from, board, &self.directions)
            .into_iter()
            .map(|pos| Move::new(MoveType::Normal, from, pos))
            .collect()
    }
}
