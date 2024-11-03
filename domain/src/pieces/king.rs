use crate::{board::Board, direction::Direction, position::Position, Color};

use super::moveable::{Move, MoveType, Moveable};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct King {
    pub directions: [Direction; 8],
}

impl Default for King {
    fn default() -> Self {
        Self::new()
    }
}

impl King {
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

impl Moveable for King {
    fn get_moves(
        &self,
        color: Color,
        _has_moved: bool,
        from: Position,
        board: &Board,
    ) -> Vec<super::moveable::Move> {
        self.move_positions(color, from, board)
            .into_iter()
            .map(|to| Move::new(MoveType::Normal, from, to))
            .collect()
    }
}

impl King {
    fn move_positions(&self, color: Color, from: Position, board: &Board) -> Vec<Position> {
        self.directions
            .iter()
            .filter_map(|dir| {
                let to = from + *dir;
                if !board.is_inside(&to) {
                    return None;
                }

                match board.get(&to) {
                    None => Some(to),
                    Some(piece) => match piece.piece_color == color {
                        true => None,
                        false => Some(to),
                    },
                }
            })
            .collect()
    }
}
