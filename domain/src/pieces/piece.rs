use crate::{board::Board, position::Position, Color};

use super::{Bishop, King, Knight, Move, Moveable, Pawn, Queen, Rook};

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn(Pawn),
    Bishop(Bishop),
    Knight(Knight),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

impl PartialEq for PieceType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PieceType::King(_), PieceType::King(_)) => true,
            (PieceType::Rook(_), PieceType::Rook(_)) => true,
            (PieceType::Bishop(_), PieceType::Bishop(_)) => true,
            (PieceType::Pawn(_), PieceType::Pawn(_)) => true,
            (PieceType::Queen(_), PieceType::Queen(_)) => true,
            (PieceType::Knight(_), PieceType::Knight(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub piece_color: Color,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(piece_type: PieceType, piece_color: Color) -> Self {
        Self {
            piece_type,
            piece_color,
            has_moved: false,
        }
    }
}

impl Moveable for Piece {
    fn get_moves(&self, color: Color, has_moved: bool, from: Position, board: &Board) -> Vec<Move> {
        match self.piece_type {
            PieceType::Pawn(pawn) => pawn.get_moves(color, has_moved, from, board),
            PieceType::Rook(rook) => rook.get_moves(color, has_moved, from, board),
            PieceType::King(king) => king.get_moves(color, has_moved, from, board),
            PieceType::Queen(queen) => queen.get_moves(color, has_moved, from, board),
            PieceType::Bishop(bishop) => bishop.get_moves(color, has_moved, from, board),
            PieceType::Knight(knight) => knight.get_moves(color, has_moved, from, board),
        }
    }

    fn can_capture_opponent_king(
        &self,
        color: Color,
        has_moved: bool,
        from: Position,
        board: &Board,
    ) -> bool {
        match self.piece_type {
            PieceType::Pawn(pawn) => pawn.can_capture_opponent_king(color, has_moved, from, board),
            PieceType::Rook(rook) => rook.can_capture_opponent_king(color, has_moved, from, board),
            PieceType::King(king) => king.can_capture_opponent_king(color, has_moved, from, board),
            PieceType::Queen(queen) => {
                queen.can_capture_opponent_king(color, has_moved, from, board)
            }
            PieceType::Bishop(bishop) => {
                bishop.can_capture_opponent_king(color, has_moved, from, board)
            }
            PieceType::Knight(knight) => {
                knight.can_capture_opponent_king(color, has_moved, from, board)
            }
        }
    }
}
