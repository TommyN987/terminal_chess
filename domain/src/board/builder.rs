use crate::{
    pieces::{Bishop, King, Knight, Pawn, Piece, PieceKind, PieceType, Queen, Rook},
    Color,
};

use super::{Board, Direction, Position};

pub struct BoardBuilder {
    board: Board,
}

impl BoardBuilder {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
        }
    }

    pub fn add_piece(mut self, pos: Position, piece_kind: PieceKind, color: Color) -> Self {
        let piece = match piece_kind {
            PieceKind::Pawn => match color {
                Color::White => Piece::new(PieceType::Pawn(Pawn::new(Direction::North)), color),
                Color::Black => Piece::new(PieceType::Pawn(Pawn::new(Direction::South)), color),
            },

            PieceKind::Knight => Piece::new(PieceType::Knight(Knight), color),
            PieceKind::Bishop => Piece::new(PieceType::Bishop(Bishop), color),
            PieceKind::Rook => Piece::new(PieceType::Rook(Rook), color),
            PieceKind::Queen => Piece::new(PieceType::Queen(Queen), color),
            PieceKind::King => Piece::new(PieceType::King(King), color),
        };

        self.board.set(&pos, Some(piece));
        self
    }

    pub fn build(self) -> Board {
        self.board
    }
}
