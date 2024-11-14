use std::fmt::Display;

use crate::{
    direction::Direction,
    pieces::{Bishop, King, Knight, Moveable, Pawn, Piece, PieceType, Queen, Rook},
    player::Player,
    position::Position,
    Color,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Board {
    pub fields: [[Option<Piece>; 8]; 8],
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

impl Board {
    pub fn new() -> Self {
        let mut board = Self::default();
        board.add_starting_pieces();
        board
    }

    pub fn get(&self, pos: &Position) -> Option<Piece> {
        self.fields[pos.row as usize][pos.column as usize]
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
                if let Some(piece) = self.get(pos) {
                    piece.can_capture_opponent_king(piece.piece_color, piece.has_moved, *pos, self)
                } else {
                    false
                }
            })
    }

    pub fn piece_positions_for_player(&self, player: &Player) -> Vec<Position> {
        self.piece_positions()
            .into_iter()
            .filter(|pos| match self.get(pos) {
                None => false,
                Some(piece) => piece.piece_color == player.color,
            })
            .collect()
    }
}

impl Board {
    fn piece_positions(&self) -> Vec<Position> {
        self.fields
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, cell)| {
                    cell.as_ref().map(|_| Position::new(i as i8, j as i8))
                })
            })
            .collect()
    }

    fn add_starting_pieces(&mut self) {
        self.set(
            &Position::new(0, 0),
            Some(Piece::new(PieceType::Rook(Rook::new()), Color::Black)),
        );
        self.set(
            &Position::new(0, 1),
            Some(Piece::new(PieceType::Knight(Knight), Color::Black)),
        );
        self.set(
            &Position::new(0, 2),
            Some(Piece::new(PieceType::Bishop(Bishop::new()), Color::Black)),
        );
        self.set(
            &Position::new(0, 3),
            Some(Piece::new(PieceType::Queen(Queen::new()), Color::Black)),
        );
        self.set(
            &Position::new(0, 4),
            Some(Piece::new(PieceType::King(King::new()), Color::Black)),
        );
        self.set(
            &Position::new(0, 5),
            Some(Piece::new(PieceType::Bishop(Bishop::new()), Color::Black)),
        );
        self.set(
            &Position::new(0, 6),
            Some(Piece::new(PieceType::Knight(Knight), Color::Black)),
        );
        self.set(
            &Position::new(0, 7),
            Some(Piece::new(PieceType::Rook(Rook::new()), Color::Black)),
        );

        for i in 0..=7 {
            self.set(
                &Position::new(1, i),
                Some(Piece::new(
                    PieceType::Pawn(Pawn::new(Direction::South)),
                    Color::Black,
                )),
            );
            self.set(
                &Position::new(6, i),
                Some(Piece::new(
                    PieceType::Pawn(Pawn::new(Direction::North)),
                    Color::White,
                )),
            );
        }

        self.set(
            &Position::new(7, 0),
            Some(Piece::new(PieceType::Rook(Rook::new()), Color::White)),
        );
        self.set(
            &Position::new(7, 1),
            Some(Piece::new(PieceType::Knight(Knight), Color::White)),
        );
        self.set(
            &Position::new(7, 2),
            Some(Piece::new(PieceType::Bishop(Bishop::new()), Color::White)),
        );
        self.set(
            &Position::new(7, 3),
            Some(Piece::new(PieceType::Queen(Queen::new()), Color::White)),
        );
        self.set(
            &Position::new(7, 4),
            Some(Piece::new(PieceType::King(King::new()), Color::White)),
        );
        self.set(
            &Position::new(7, 5),
            Some(Piece::new(PieceType::Bishop(Bishop::new()), Color::White)),
        );
        self.set(
            &Position::new(7, 6),
            Some(Piece::new(PieceType::Knight(Knight), Color::White)),
        );
        self.set(
            &Position::new(7, 7),
            Some(Piece::new(PieceType::Rook(Rook::new()), Color::White)),
        );
    }
}

#[cfg(test)]
mod tests {
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
            Some(Piece::new(PieceType::Bishop(Bishop::new()), Color::Black)),
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
            Some(Piece::new(PieceType::Bishop(Bishop::new()), Color::White)),
        );

        let is_white_in_check = board.is_in_check(white);
        let is_black_in_check = board.is_in_check(black);

        assert!(!is_white_in_check);
        assert!(is_black_in_check);
    }
}
