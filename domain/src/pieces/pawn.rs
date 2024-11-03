use crate::{board::Board, direction::Direction, position::Position, Color};

use super::moveable::{Move, MoveType, Moveable};

pub struct Pawn {
    color: Color,
    forward: Direction,
    has_moved: bool,
}

impl Moveable for Pawn {
    fn get_moves(&self, from: Position, board: &Board) -> Vec<Move> {
        vec![
            self.forward_moves(from, board),
            self.diagonal_moves(from, board),
        ]
        .concat()
    }
}

impl Pawn {
    pub fn new(color: Color) -> Self {
        let forward = match color {
            Color::White => Direction::North,
            Color::Black => Direction::South,
        };

        Self {
            color,
            forward,
            has_moved: false,
        }
    }

    fn can_move_to(&self, pos: &Position, board: &Board) -> bool {
        board.is_inside(pos) && board.get(pos).is_none()
    }

    fn can_capture_at(&self, pos: &Position, board: &Board) -> bool {
        if !board.is_inside(pos) {
            return false;
        }

        match board.get(pos) {
            None => false,
            Some(piece) => piece.piece_color != self.color,
        }
    }

    fn forward_moves(&self, from: Position, board: &Board) -> Vec<Move> {
        let mut result = vec![];
        let one_move_position = from + self.forward;

        if self.can_move_to(&one_move_position, board) {
            result.push(Move::new(MoveType::Normal, from, one_move_position));

            let two_move_position = one_move_position + self.forward;

            if !self.has_moved && self.can_move_to(&two_move_position, board) {
                result.push(Move::new(MoveType::DoublePawn, from, two_move_position));
            }
        }

        result
    }

    fn diagonal_moves(&self, from: Position, board: &Board) -> Vec<Move> {
        let mut result = vec![];
        for dir in [Direction::East, Direction::West] {
            let to = from + self.forward + dir;

            if self.can_capture_at(&to, board) {
                result.push(Move::new(MoveType::Normal, from, to));
            }
        }

        result
    }
}
