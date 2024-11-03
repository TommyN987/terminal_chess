use crate::{
    pieces::{Piece, PieceType},
    position::Position,
    Color,
};

#[derive(Debug, Default, Clone, PartialEq)]
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

    pub fn set(&mut self, pos: &Position, piece: Option<Piece>) {
        self.fields[pos.row as usize][pos.column as usize] = piece;
    }

    pub fn is_inside(&self, pos: &Position) -> bool {
        pos.row >= 0 && pos.row < 8 && pos.column >= 0 && pos.column < 8
    }
}

impl Board {
    fn add_starting_pieces(&mut self) {
        self.set(
            &Position::new(0, 0),
            Some(Piece::new(PieceType::Rook, Color::Black)),
        );
        self.set(
            &Position::new(0, 1),
            Some(Piece::new(PieceType::Knight, Color::Black)),
        );
        self.set(
            &Position::new(0, 2),
            Some(Piece::new(PieceType::Bishop, Color::Black)),
        );
        self.set(
            &Position::new(0, 3),
            Some(Piece::new(PieceType::Queen, Color::Black)),
        );
        self.set(
            &Position::new(0, 4),
            Some(Piece::new(PieceType::King, Color::Black)),
        );
        self.set(
            &Position::new(0, 5),
            Some(Piece::new(PieceType::Bishop, Color::Black)),
        );
        self.set(
            &Position::new(0, 6),
            Some(Piece::new(PieceType::Knight, Color::Black)),
        );
        self.set(
            &Position::new(0, 7),
            Some(Piece::new(PieceType::Rook, Color::Black)),
        );

        for i in 0..=7 {
            self.set(
                &Position::new(1, i),
                Some(Piece::new(PieceType::Pawn, Color::Black)),
            );
            self.set(
                &Position::new(6, i),
                Some(Piece::new(PieceType::Pawn, Color::White)),
            );
        }

        self.set(
            &Position::new(7, 0),
            Some(Piece::new(PieceType::Rook, Color::White)),
        );
        self.set(
            &Position::new(7, 1),
            Some(Piece::new(PieceType::Knight, Color::White)),
        );
        self.set(
            &Position::new(7, 2),
            Some(Piece::new(PieceType::Bishop, Color::White)),
        );
        self.set(
            &Position::new(7, 3),
            Some(Piece::new(PieceType::Queen, Color::White)),
        );
        self.set(
            &Position::new(7, 4),
            Some(Piece::new(PieceType::King, Color::White)),
        );
        self.set(
            &Position::new(7, 5),
            Some(Piece::new(PieceType::Bishop, Color::White)),
        );
        self.set(
            &Position::new(7, 6),
            Some(Piece::new(PieceType::Knight, Color::White)),
        );
        self.set(
            &Position::new(7, 7),
            Some(Piece::new(PieceType::Rook, Color::White)),
        );
    }
}
