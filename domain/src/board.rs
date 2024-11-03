use crate::{
    direction::Direction,
    pieces::{Bishop, King, Knight, Moveable, Pawn, Piece, PieceType, Queen, Rook},
    player::Player,
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

    pub fn is_in_check(&self, player: Player) -> bool {
        self.piece_positions_for_player(player).iter().any(|pos| {
            if let Some(piece) = self.get(pos) {
                return piece.can_capture_opponent_king(
                    piece.piece_color,
                    piece.has_moved,
                    *pos,
                    self,
                );
            } else {
                false
            }
        })
    }
}

impl Board {
    fn piece_positions(&self) -> Vec<Position> {
        let mut positions = vec![];
        for (i, row) in self.fields.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if let Some(_) = cell {
                    positions.push(Position::new(i as i8, j as i8));
                }
            }
        }
        positions
    }

    fn piece_positions_for_player(&self, player: Player) -> Vec<Position> {
        self.piece_positions()
            .into_iter()
            .filter(|pos| match self.get(pos) {
                None => false,
                Some(piece) => piece.piece_color == player.color,
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
