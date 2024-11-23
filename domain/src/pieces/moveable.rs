use derive_new::new;

use crate::{board::Board, direction::Direction, player::Player, position::Position, Color};

use super::{King, Pawn, Piece, PieceType};

pub trait Moveable {
    fn get_moves(&self, color: Color, has_moved: bool, from: Position, board: &Board) -> Vec<Move>;

    fn reachable_positions_in_direction(
        &self,
        from: Position,
        board: &Board,
        dir: &Direction,
    ) -> Vec<Position> {
        let mut result = vec![];
        if let Some(current_piece) = board.get(&from) {
            let mut pos = from + *dir;
            while board.is_inside(&(pos)) {
                match board.get(&pos) {
                    None => result.push(pos),
                    Some(piece) => {
                        if current_piece.piece_color != piece.piece_color {
                            result.push(pos);
                        }
                        break;
                    }
                }
                pos += *dir;
            }
        }
        result
    }

    fn reachable_positions_in_many_directions(
        &self,
        from: Position,
        board: &Board,
        dirs: &[Direction],
    ) -> Vec<Position> {
        dirs.iter()
            .flat_map(|dir| self.reachable_positions_in_direction(from, board, dir))
            .collect()
    }

    fn can_capture_opponent_king(
        &self,
        color: Color,
        has_moved: bool,
        from: Position,
        board: &Board,
    ) -> bool {
        self.get_moves(color, has_moved, from, board)
            .iter()
            .any(|m| match board.get(&m.to) {
                None => false,
                Some(piece) => piece.piece_type == PieceType::King(King::default()),
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveType {
    Normal,
    ShortCastle,
    LongCastle,
    DoublePawn,
    EnPassant,
    Promotion,
}

#[derive(Debug, Clone, Copy, PartialEq, new)]
pub struct Move {
    pub move_type: MoveType,
    pub from: Position,
    pub to: Position,
}

impl Move {
    pub fn execute(&self, board: &mut Board, promotion_piece: Option<Piece>) -> bool {
        let is_capture = board.get(&self.to).is_some();
        let mut is_pawn_move = false;

        if let Some(mut piece) = board.get(&self.from) {
            piece.has_moved = true;
            if piece.piece_type == PieceType::Pawn(Pawn::new(Direction::North)) {
                is_pawn_move = true;
            }

            match self.move_type {
                MoveType::ShortCastle => {
                    let rook_move = Move::new(
                        MoveType::Normal,
                        Position::from((self.from.row, 7)),
                        Position::from((self.from.row, 5)),
                    );
                    let mut king_move = self.clone();
                    king_move.move_type = MoveType::Normal;
                    king_move.execute(board, None);
                    rook_move.execute(board, None);
                }
                MoveType::LongCastle => {
                    let rook_move = Move::new(
                        MoveType::Normal,
                        Position::from((self.from.row, 0)),
                        Position::from((self.from.row, 3)),
                    );
                    let mut king_move = self.clone();
                    king_move.move_type = MoveType::Normal;
                    king_move.execute(board, None);
                    rook_move.execute(board, None);
                }
                MoveType::DoublePawn => {
                    board.set_en_passant_square(&self, &piece.piece_color);
                    board.set(&self.to, Some(piece));
                    board.set(&self.from, None);
                }
                MoveType::EnPassant => {
                    board.set(&self.to, Some(piece));
                    board.set(&self.from, None);
                    board.set(&Position::from((self.from.row, self.to.column)), None);
                }
                _ => match promotion_piece {
                    None => {
                        piece.has_moved = true;
                        board.set(&self.to, Some(piece));
                        board.set(&self.from, None);
                    }
                    Some(new_piece) => {
                        board.set(&self.to, Some(new_piece));
                        board.set(&self.from, None);
                    }
                },
            }
        }

        if self.move_type != MoveType::DoublePawn {
            board.clear_en_passant_squares();
        }

        is_capture || is_pawn_move
    }

    pub fn is_legal(&self, board: &Board) -> bool {
        match self.move_type {
            MoveType::ShortCastle => self.is_short_castling_legal(board),
            MoveType::LongCastle => self.is_long_castling_legal(board),
            _ => self.is_normal_move_legal(board),
        }
    }

    fn is_normal_move_legal(&self, board: &Board) -> bool {
        if let Some(piece) = board.get(&self.from) {
            let player = Player::new(piece.piece_color);
            let mut cloned_board = board.clone();
            self.execute(&mut cloned_board, None);
            return !cloned_board.is_in_check(player);
        }
        false
    }

    fn is_short_castling_legal(&self, board: &Board) -> bool {
        let mut is_legal = false;
        if let Some(piece) = board.get(&self.from) {
            let player = Player::new(piece.piece_color);
            let columns = [5, 6];

            is_legal = columns.into_iter().all(|column| {
                let mut cloned_board = board.clone();
                let between_move = Move::new(
                    MoveType::Normal,
                    self.from,
                    Position::new(self.to.row, column),
                );
                between_move.execute(&mut cloned_board, None);
                !cloned_board.is_in_check(player)
            }) && !board.is_in_check(player);
        }
        is_legal
    }

    fn is_long_castling_legal(&self, board: &Board) -> bool {
        let mut is_legal = false;
        if let Some(piece) = board.get(&self.from) {
            let player = Player::new(piece.piece_color);
            let columns = [2, 3];

            is_legal = columns.into_iter().all(|column| {
                let mut cloned_board = board.clone();
                let between_move = Move::new(
                    MoveType::Normal,
                    self.from,
                    Position::new(self.to.row, column),
                );
                between_move.execute(&mut cloned_board, None);
                !cloned_board.is_in_check(player)
            }) && !board.is_in_check(player);
        }
        is_legal
    }
}

#[cfg(test)]
mod tests {
    use crate::pieces::{Bishop, Pawn, Piece};

    use super::*;

    #[test]
    fn test_execute_works() {
        let mut board = Board::new();
        let from = Position::from((6, 0));
        let to = Position::from((5, 0));

        let m = Move::new(MoveType::Normal, from, to);

        m.execute(&mut board, None);

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
            Some(Piece::new(PieceType::Bishop(Bishop::new()), Color::Black)),
        );

        assert!(!m.is_legal(&board));
    }
}
