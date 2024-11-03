use crate::{board::Board, position::Position, Color};

use super::{Bishop, King, Knight, Move, Moveable, Pawn, Queen, Rook};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn(Pawn),
    Bishop(Bishop),
    Knight(Knight),
    Rook(Rook),
    Queen(Queen),
    King(King),
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
}
