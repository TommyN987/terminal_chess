use derive_new::new;

use crate::{
    board::{Board, Position},
    game::{Color, Player},
    pieces::{Piece, PieceKind, PromotionPiece},
};

#[derive(Debug, Clone, PartialEq, new)]
pub struct Move {
    pub move_type: MoveType,
    pub from: Position,
    pub to: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveType {
    Normal,
    ShortCastle,
    LongCastle,
    DoublePawn,
    EnPassant,
    Promotion(PromotionPiece),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveRecord {
    pub mv: Move,
    pub piece_moved: PieceKind,
    pub piece_captured: Option<PieceKind>,
    pub is_check: bool,
}

impl Move {
    // TODO: Implement a slimmer `.simulate()` method and make `.execute() consuming`
    pub fn execute(&self, board: &mut Board) -> MoveRecord {
        let mut piece = board[&self.from].expect("Invalid move: No piece at origin square");
        let current_player = piece.piece_color;
        let piece_moved = PieceKind::from(&piece.piece_type);
        let piece_captured = board[&self.to].map(|piece| piece.piece_type.into());

        piece.has_moved = true;

        match self.move_type {
            MoveType::Normal => self.handle_normal_move(board, piece),
            MoveType::DoublePawn => self.handle_double_pawn(board, piece),
            MoveType::EnPassant => self.handle_en_passant(board, piece),
            MoveType::Promotion(ref kind) => self.handle_promotion(board, kind, piece.piece_color),
            MoveType::ShortCastle | MoveType::LongCastle => self.handle_castling(board, piece),
        };

        let is_check = board.is_in_check(Player::new(current_player));

        if self.move_type != MoveType::DoublePawn {
            board.clear_en_passant_squares();
        }

        MoveRecord {
            mv: self.clone(),
            piece_moved,
            piece_captured,
            is_check,
        }
    }

    pub fn is_legal(&self, board: &Board) -> bool {
        match self.move_type {
            MoveType::ShortCastle | MoveType::LongCastle => self.is_castling_legal(board),
            _ => self.is_move_legal(board),
        }
    }
}

impl Move {
    fn handle_normal_move(&self, board: &mut Board, piece: Piece) {
        board.set(&self.to, Some(piece));
        board.set(&self.from, None);
    }

    fn handle_castling(&self, board: &mut Board, king: Piece) {
        let rook_move = match self.move_type {
            MoveType::ShortCastle => Self::new(
                MoveType::Normal,
                Position::from((self.from.row, 7)),
                Position::from((self.from.row, 5)),
            ),
            MoveType::LongCastle => Self::new(
                MoveType::Normal,
                Position::from((self.from.row, 0)),
                Position::from((self.from.row, 3)),
            ),
            _ => return,
        };
        self.handle_normal_move(board, king);
        rook_move.execute(board);
    }

    fn handle_double_pawn(&self, board: &mut Board, piece: Piece) {
        board.set_en_passant_square(self, &piece.piece_color);
        self.handle_normal_move(board, piece);
    }

    fn handle_en_passant(&self, board: &mut Board, piece: Piece) {
        board.set(&Position::from((self.from.row, self.to.column)), None);
        self.handle_normal_move(board, piece);
    }

    fn handle_promotion(&self, board: &mut Board, promotion_piece: &PromotionPiece, color: Color) {
        let promoted_piece = Piece {
            piece_type: promotion_piece.into(),
            piece_color: color,
            has_moved: true,
        };
        self.handle_normal_move(board, promoted_piece);
    }

    fn is_move_legal(&self, board: &Board) -> bool {
        board.get(&self.from).map_or(false, |piece| {
            let player = Player::new(piece.piece_color);
            let mut cloned_board = board.clone();
            self.execute(&mut cloned_board);
            !cloned_board.is_in_check(player)
        })
    }

    fn is_castling_legal(&self, board: &Board) -> bool {
        let piece = match board.get(&self.from) {
            Some(piece) => piece,
            None => return false,
        };

        let player = Player::new(piece.piece_color);
        let columns = match self.move_type {
            MoveType::LongCastle => [2, 3],
            MoveType::ShortCastle => [5, 6],
            _ => return false,
        };

        !board.is_in_check(player)
            && columns.into_iter().all(|column| {
                let mut cloned_board = board.clone();
                let between_move = Move::new(
                    MoveType::Normal,
                    self.from,
                    Position::new(self.to.row, column),
                );
                between_move.execute(&mut cloned_board);
                !cloned_board.is_in_check(player)
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::Direction,
        pieces::{Bishop, Pawn, Piece, PieceType},
    };

    use super::*;

    #[test]
    fn test_execute_works() {
        let mut board = Board::new();
        let from = Position::from((6, 0));
        let to = Position::from((5, 0));

        let m = Move::new(MoveType::Normal, from, to);

        m.execute(&mut board);

        let mut moved_pawn = Piece::new(PieceType::Pawn(Pawn::new(Direction::North)), Color::White);
        moved_pawn.has_moved = true;

        assert!(board.get(&from).is_none());
        assert_eq!(board.get(&to), Some(moved_pawn));
    }

    #[test]
    fn test_is_legal_works() {
        let mut board = Board::new();

        let from = Position::from((6, 3));
        let to = Position::from((5, 3));

        let m = Move::new(MoveType::Normal, from, to);

        assert!(m.is_legal(&board));

        board.set(
            &Position::from((4, 1)),
            Some(Piece::new(PieceType::Bishop(Bishop), Color::Black)),
        );

        assert!(!m.is_legal(&board));
    }
}
