use crate::{
    pieces::{Piece, PieceType},
    position::Position,
    Color,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Board {
    fields: [[Option<Piece>; 8]; 8],
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

    pub fn set(&mut self, pos: &Position, piece: Piece) {
        self.fields[pos.row as usize][pos.column as usize] = Some(piece);
    }

    pub fn is_inside(&self, pos: &Position) -> bool {
        pos.row >= 0 && pos.row < 8 && pos.column >= 0 && pos.column < 8
    }
}

impl Board {
    fn add_starting_pieces(&mut self) {
        self.set(
            &Position::new(0, 0),
            Piece::new(PieceType::Rook, Color::Black),
        );
        self.set(
            &Position::new(0, 1),
            Piece::new(PieceType::Knight, Color::Black),
        );
        self.set(
            &Position::new(0, 2),
            Piece::new(PieceType::Bishop, Color::Black),
        );
        self.set(
            &Position::new(0, 3),
            Piece::new(PieceType::Queen, Color::Black),
        );
        self.set(
            &Position::new(0, 4),
            Piece::new(PieceType::King, Color::Black),
        );
        self.set(
            &Position::new(0, 5),
            Piece::new(PieceType::Bishop, Color::Black),
        );
        self.set(
            &Position::new(0, 6),
            Piece::new(PieceType::Knight, Color::Black),
        );
        self.set(
            &Position::new(0, 7),
            Piece::new(PieceType::Rook, Color::Black),
        );

        for i in 0..=7 {
            self.set(
                &Position::new(1, i),
                Piece::new(PieceType::Pawn, Color::Black),
            );
            self.set(
                &Position::new(6, i),
                Piece::new(PieceType::Pawn, Color::White),
            );
        }

        self.set(
            &Position::new(7, 0),
            Piece::new(PieceType::Rook, Color::White),
        );
        self.set(
            &Position::new(7, 1),
            Piece::new(PieceType::Knight, Color::White),
        );
        self.set(
            &Position::new(7, 2),
            Piece::new(PieceType::Bishop, Color::White),
        );
        self.set(
            &Position::new(7, 3),
            Piece::new(PieceType::Queen, Color::White),
        );
        self.set(
            &Position::new(7, 4),
            Piece::new(PieceType::King, Color::White),
        );
        self.set(
            &Position::new(7, 5),
            Piece::new(PieceType::Bishop, Color::White),
        );
        self.set(
            &Position::new(7, 6),
            Piece::new(PieceType::Knight, Color::White),
        );
        self.set(
            &Position::new(7, 7),
            Piece::new(PieceType::Rook, Color::White),
        );
    }
}
