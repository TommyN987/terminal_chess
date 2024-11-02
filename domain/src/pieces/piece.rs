use crate::Color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
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
