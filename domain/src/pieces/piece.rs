use crate::{board::Board, position::Position, Color};

use super::{Bishop, King, Knight, Move, Moveable, Pawn, Queen, Rook};

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

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn(Pawn),
    Bishop(Bishop),
    Knight(Knight),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

impl PieceType {
    pub fn as_index(&self) -> usize {
        match self {
            PieceType::Pawn(_) => 0,
            PieceType::Knight(_) => 1,
            PieceType::Bishop(_) => 2,
            PieceType::Rook(_) => 3,
            PieceType::Queen(_) => 4,
            PieceType::King(_) => 5,
        }
    }
}

impl PartialEq for PieceType {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (PieceType::King(_), PieceType::King(_))
                | (PieceType::Rook(_), PieceType::Rook(_))
                | (PieceType::Bishop(_), PieceType::Bishop(_))
                | (PieceType::Pawn(_), PieceType::Pawn(_))
                | (PieceType::Queen(_), PieceType::Queen(_))
                | (PieceType::Knight(_), PieceType::Knight(_))
        )
    }
}

pub struct PieceCounter {
    white: [u8; 6],
    black: [u8; 6],
    total: u8,
}

impl PieceCounter {
    pub fn new() -> Self {
        Self {
            white: [0; 6],
            black: [0; 6],
            total: 0,
        }
    }

    pub fn increment(&mut self, color: &Color, piece: &PieceType) {
        match color {
            Color::White => self.white[piece.as_index()] += 1,
            Color::Black => self.black[piece.as_index()] += 1,
        };
        self.total += 1;
    }

    pub fn get_white(&self, piece: &PieceType) -> u8 {
        self.white[piece.as_index()]
    }

    pub fn get_black(&self, piece: &PieceType) -> u8 {
        self.black[piece.as_index()]
    }

    pub fn get_total(&self) -> u8 {
        self.total
    }
}

#[cfg(test)]
mod tests {
    use crate::direction::Direction;

    use super::*;

    #[test]
    fn test_piece_can_capture_opponent_king_works() {
        let mut board = Board::default();
        let opponent_king_position = Position::from((4, 4));
        let white_pieces = vec![
            (
                Piece::new(PieceType::Pawn(Pawn::new(Direction::North)), Color::White),
                Position::from((5, 5)),
            ),
            (
                Piece::new(PieceType::Knight(Knight), Color::White),
                Position::from((2, 3)),
            ),
            (
                Piece::new(PieceType::Bishop(Bishop::new()), Color::White),
                Position::from((7, 1)),
            ),
            (
                Piece::new(PieceType::Rook(Rook::new()), Color::White),
                Position::from((4, 7)),
            ),
            (
                Piece::new(PieceType::Queen(Queen::new()), Color::White),
                Position::from((4, 0)),
            ),
            (
                Piece::new(PieceType::King(King::new()), Color::White),
                Position::from((5, 4)),
            ),
        ];

        let non_capturing_pieces = vec![
            (
                Piece::new(PieceType::Pawn(Pawn::new(Direction::North)), Color::White),
                Position::from((6, 0)),
            ),
            (
                Piece::new(PieceType::Queen(Queen::new()), Color::White),
                Position::from((7, 7)),
            ),
            (
                Piece::new(PieceType::Bishop(Bishop::new()), Color::White),
                Position::from((7, 4)),
            ),
        ];

        board.set(
            &opponent_king_position,
            Some(Piece::new(PieceType::King(King::new()), Color::Black)),
        );

        white_pieces.iter().for_each(|(piece, pos)| {
            board.set(pos, Some(*piece));
        });

        non_capturing_pieces.iter().for_each(|(piece, pos)| {
            board.set(pos, Some(*piece));
        });

        for (piece, pos) in white_pieces {
            assert!(
                piece.can_capture_opponent_king(Color::White, true, pos, &board),
                "{:?} should be able to capture opponent king from position {:?}",
                piece,
                pos
            );
        }

        for (piece, pos) in non_capturing_pieces {
            assert!(
                !piece.can_capture_opponent_king(Color::White, true, pos, &board),
                "{:?} should not be able to capture opponent king from position {:?}",
                piece,
                pos
            );
        }
    }
}
