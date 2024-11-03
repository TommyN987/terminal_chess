use crate::{board::Board, direction::Direction, position::Position, Color};

use super::moveable::{Move, MoveType, Moveable};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pawn {
    forward: Direction,
}

impl Moveable for Pawn {
    fn get_moves(&self, color: Color, has_moved: bool, from: Position, board: &Board) -> Vec<Move> {
        [
            self.forward_moves(has_moved, from, board),
            self.diagonal_moves(color, from, board),
        ]
        .concat()
    }
}

impl Pawn {
    pub fn new(forward: Direction) -> Self {
        Self { forward }
    }

    fn can_move_to(&self, pos: &Position, board: &Board) -> bool {
        board.is_inside(pos) && board.get(pos).is_none()
    }

    fn can_capture_at(&self, color: Color, pos: &Position, board: &Board) -> bool {
        if !board.is_inside(pos) {
            return false;
        }

        match board.get(pos) {
            None => false,
            Some(piece) => piece.piece_color != color,
        }
    }

    fn forward_moves(&self, has_moved: bool, from: Position, board: &Board) -> Vec<Move> {
        let mut result = vec![];
        let one_move_position = from + self.forward;

        if self.can_move_to(&one_move_position, board) {
            result.push(Move::new(MoveType::Normal, from, one_move_position));

            let two_move_position = one_move_position + self.forward;

            if !has_moved && self.can_move_to(&two_move_position, board) {
                result.push(Move::new(MoveType::DoublePawn, from, two_move_position));
            }
        }

        result
    }

    fn diagonal_moves(&self, color: Color, from: Position, board: &Board) -> Vec<Move> {
        let mut result = vec![];
        for dir in [Direction::East, Direction::West] {
            let to = from + self.forward + dir;

            if self.can_capture_at(color, &to, board) {
                result.push(Move::new(MoveType::Normal, from, to));
            }
        }

        result
    }
}
