use std::fmt::Display;

use derive_new::new;

use crate::{
    game::{Color, Player},
    moves::{Move, Moveable},
    pieces::{Piece, PieceCounter, PieceKind, PieceType},
};

use super::{BoardBuilder, Position};

#[derive(Debug, Default, Clone, PartialEq, new)]
pub(crate) struct EnPassantSquare {
    white: Option<Position>,
    black: Option<Position>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Board {
    pub fields: [[Option<Piece>; 8]; 8],
    en_passant_square: EnPassantSquare,
}

impl Board {
    pub fn new() -> Self {
        Self::init_starting_position()
    }

    pub fn set(&mut self, pos: &Position, piece: Option<Piece>) {
        self.fields[pos.row as usize][pos.column as usize] = piece;
    }

    pub fn is_inside(&self, pos: &Position) -> bool {
        pos.row >= 0 && pos.row < 8 && pos.column >= 0 && pos.column < 8
    }

    pub fn is_in_check(&self, player: Player) -> bool {
        self.piece_positions_for_player(&player.opponent())
            .iter()
            .any(|pos| {
                if let Some(piece) = self[pos] {
                    piece.can_capture_opponent_king(piece.piece_color, piece.has_moved, *pos, self)
                } else {
                    false
                }
            })
    }

    pub fn piece_positions_for_player(&self, player: &Player) -> Vec<Position> {
        self.piece_positions()
            .into_iter()
            .filter(|pos| match self[pos] {
                None => false,
                Some(piece) => piece.piece_color == player.color,
            })
            .collect()
    }

    pub fn piece_positions(&self) -> Vec<Position> {
        self.fields
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, cell)| cell.as_ref().map(|_| Position::from((i, j))))
            })
            .collect()
    }

    pub fn get_en_passant_square(&self, player: &Color) -> Option<Position> {
        match player {
            Color::White => self.en_passant_square.white,
            Color::Black => self.en_passant_square.black,
        }
    }

    pub fn set_en_passant_square(&mut self, m: &Move, player: &Color) {
        let en_passant_square = Position::from(((m.from.row + m.to.row) / 2, m.to.column));
        match player {
            Color::White => {
                self.en_passant_square.white = Some(en_passant_square);
                self.en_passant_square.black = None;
            }
            Color::Black => {
                self.en_passant_square.black = Some(en_passant_square);
                self.en_passant_square.white = None;
            }
        }
    }

    pub fn clear_en_passant_squares(&mut self) {
        self.en_passant_square.black = None;
        self.en_passant_square.white = None;
    }
}

impl Board {
    fn init_starting_position() -> Self {
        let builder = BoardBuilder::new();
        builder
            .add_piece(Position::new(0, 0), PieceKind::Rook, Color::Black)
            .add_piece(Position::new(0, 1), PieceKind::Knight, Color::Black)
            .add_piece(Position::new(0, 2), PieceKind::Bishop, Color::Black)
            .add_piece(Position::new(0, 3), PieceKind::Queen, Color::Black)
            .add_piece(Position::new(0, 4), PieceKind::King, Color::Black)
            .add_piece(Position::new(0, 5), PieceKind::Bishop, Color::Black)
            .add_piece(Position::new(0, 6), PieceKind::Knight, Color::Black)
            .add_piece(Position::new(0, 7), PieceKind::Rook, Color::Black)
            .add_piece(Position::new(1, 0), PieceKind::Pawn, Color::Black)
            .add_piece(Position::new(1, 1), PieceKind::Pawn, Color::Black)
            .add_piece(Position::new(1, 2), PieceKind::Pawn, Color::Black)
            .add_piece(Position::new(1, 3), PieceKind::Pawn, Color::Black)
            .add_piece(Position::new(1, 4), PieceKind::Pawn, Color::Black)
            .add_piece(Position::new(1, 5), PieceKind::Pawn, Color::Black)
            .add_piece(Position::new(1, 6), PieceKind::Pawn, Color::Black)
            .add_piece(Position::new(1, 7), PieceKind::Pawn, Color::Black)
            .add_piece(Position::new(6, 0), PieceKind::Pawn, Color::White)
            .add_piece(Position::new(6, 1), PieceKind::Pawn, Color::White)
            .add_piece(Position::new(6, 2), PieceKind::Pawn, Color::White)
            .add_piece(Position::new(6, 3), PieceKind::Pawn, Color::White)
            .add_piece(Position::new(6, 4), PieceKind::Pawn, Color::White)
            .add_piece(Position::new(6, 5), PieceKind::Pawn, Color::White)
            .add_piece(Position::new(6, 6), PieceKind::Pawn, Color::White)
            .add_piece(Position::new(6, 7), PieceKind::Pawn, Color::White)
            .add_piece(Position::new(7, 0), PieceKind::Rook, Color::White)
            .add_piece(Position::new(7, 1), PieceKind::Knight, Color::White)
            .add_piece(Position::new(7, 2), PieceKind::Bishop, Color::White)
            .add_piece(Position::new(7, 3), PieceKind::Queen, Color::White)
            .add_piece(Position::new(7, 4), PieceKind::King, Color::White)
            .add_piece(Position::new(7, 5), PieceKind::Bishop, Color::White)
            .add_piece(Position::new(7, 6), PieceKind::Knight, Color::White)
            .add_piece(Position::new(7, 7), PieceKind::Rook, Color::White)
            .build()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.fields {
            for cell in row {
                match cell {
                    None => write!(f, "[ ]")?,
                    Some(p) => write!(
                        f,
                        "[{}]",
                        match p.piece_type {
                            PieceType::Pawn(_) => "p",
                            PieceType::Rook(_) => "R",
                            PieceType::King(_) => "K",
                            PieceType::Queen(_) => "Q",
                            PieceType::Bishop(_) => "B",
                            PieceType::Knight(_) => "N",
                        }
                    )?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::pieces::Bishop;

    use super::*;

    #[test]
    fn test_is_in_check_works() {
        let mut board = Board::new();
        let white = Player::new(Color::White);
        let black = Player::new(Color::Black);

        let is_white_in_check = board.is_in_check(white);
        let is_black_in_check = board.is_in_check(black);

        assert!(!is_white_in_check);
        assert!(!is_black_in_check);

        board.set(&Position::from((6, 3)), None);
        board.set(
            &Position::from((4, 1)),
            Some(Piece::new(PieceType::Bishop(Bishop), Color::Black)),
        );

        let is_white_in_check = board.is_in_check(white);
        let is_black_in_check = board.is_in_check(black);

        assert!(is_white_in_check);
        assert!(!is_black_in_check);

        board = Board::new();

        let is_white_in_check = board.is_in_check(white);
        let is_black_in_check = board.is_in_check(black);

        assert!(!is_white_in_check);
        assert!(!is_black_in_check);

        board.set(&Position::from((1, 3)), None);
        board.set(
            &Position::from((3, 1)),
            Some(Piece::new(PieceType::Bishop(Bishop), Color::White)),
        );

        let is_white_in_check = board.is_in_check(white);
        let is_black_in_check = board.is_in_check(black);

        assert!(!is_white_in_check);
        assert!(is_black_in_check);
    }
}
