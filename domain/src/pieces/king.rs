use crate::{board::Board, direction::Direction, position::Position, Color};

use super::moveable::{Move, MoveType, Moveable};

#[derive(Debug, Clone)]
pub struct King {
    pub color: Color,
    pub has_moved: bool,
    pub directions: [Direction; 8],
}

impl King {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            has_moved: false,
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
    fn get_moves(&self, from: Position, board: &Board) -> Vec<super::moveable::Move> {
        self.move_positions(from, board)
            .into_iter()
            .map(|to| Move::new(MoveType::Normal, from, to))
            .collect()
    }
}

impl King {
    fn move_positions(&self, from: Position, board: &Board) -> Vec<Position> {
        self.directions
            .iter()
            .filter_map(|dir| {
                let to = from + *dir;
                if !board.is_inside(&to) {
                    return None;
                }

                match board.get(&to) {
                    None => Some(to),
                    Some(piece) => match piece.piece_color == self.color {
                        true => None,
                        false => Some(to),
                    },
                }
            })
            .collect()
    }
}
