use derive_new::new;

use crate::{
    board::{Board, Direction, Position},
    moves::{Move, MoveType, Moveable},
    Color,
};

use super::{King, PieceKind, PieceType};

#[derive(Debug, Clone, Copy, PartialEq, new)]
pub struct Pawn {
    forward: Direction,
}

impl Moveable for Pawn {
    fn get_moves(&self, color: Color, has_moved: bool, from: Position, board: &Board) -> Vec<Move> {
        [
            self.forward_moves(has_moved, from, board),
            self.diagonal_moves(color, from, board),
        ]
        .concat()
    }

    fn can_capture_opponent_king(
        &self,
        color: Color,
        _has_moved: bool,
        from: Position,
        board: &Board,
    ) -> bool {
        self.diagonal_moves(color, from, board)
            .iter()
            .any(|m| match board.get(&m.to) {
                None => false,
                Some(piece) => piece.piece_type == PieceType::King(King),
            })
    }
}

impl Pawn {
    fn can_move_to(&self, pos: &Position, board: &Board) -> bool {
        board.is_inside(pos) && board.get(pos).is_none()
    }

    fn can_capture_at(&self, color: Color, pos: &Position, board: &Board) -> bool {
        if !board.is_inside(pos) {
            return false;
        }

        match board.get(pos) {
            None => false,
            Some(piece) => piece.piece_color != color,
        }
    }

    fn forward_moves(&self, has_moved: bool, from: Position, board: &Board) -> Vec<Move> {
        let mut result = vec![];
        let one_move_position = from + self.forward;

        if self.can_move_to(&one_move_position, board) {
            result.push(Move::new(
                self.derive_move_type(&one_move_position),
                from,
                one_move_position,
            ));

            if !has_moved {
                let two_move_position = one_move_position + self.forward;

                if self.can_move_to(&two_move_position, board) {
                    result.push(Move::new(MoveType::DoublePawn, from, two_move_position));
                }
            }
        }

        result
    }

    fn diagonal_moves(&self, color: Color, from: Position, board: &Board) -> Vec<Move> {
        let mut result = vec![];
        for dir in [Direction::East, Direction::West] {
            let to = from + self.forward + dir;

            if let Some(en_passant_square) = board.get_en_passant_square(&color.opponent()) {
                if to == en_passant_square {
                    result.push(Move::new(MoveType::EnPassant, from, to));
                    continue;
                }
            }

            if self.can_capture_at(color, &to, board) {
                result.push(Move::new(self.derive_move_type(&to), from, to));
            }
        }

        result
    }

    fn derive_move_type(&self, to: &Position) -> MoveType {
        if to.row == 0 || to.row == 7 {
            return MoveType::Promotion(PieceKind::Queen);
        }
        MoveType::Normal
    }
}

#[cfg(test)]
mod tests {
    use crate::pieces::Piece;

    use super::*;

    #[test]
    fn test_pawn_initial_double_move() {
        let board = Board::new();
        let white_pawn = Pawn::new(Direction::North);
        let start_pos = Position::from((6, 4));
        let moves = white_pawn.get_moves(Color::White, false, start_pos, &board);

        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&Move::new(
            MoveType::Normal,
            start_pos,
            Position::from((5, 4))
        )));
        assert!(moves.contains(&Move::new(
            MoveType::DoublePawn,
            start_pos,
            Position::from((4, 4))
        )));
    }

    #[test]
    fn test_pawn_single_forward_move_after_initial() {
        let mut board = Board::new();
        let white_pawn = Pawn::new(Direction::North);
        let start_pos = Position::from((5, 4));
        board.set(
            &start_pos,
            Some(Piece::new(PieceType::Pawn(white_pawn), Color::White)),
        );

        let moves = white_pawn.get_moves(Color::White, true, start_pos, &board);

        assert_eq!(moves.len(), 1);
        assert!(moves.contains(&Move::new(
            MoveType::Normal,
            start_pos,
            Position::from((4, 4))
        )));
    }

    #[test]
    fn test_pawn_cannot_move_forward_if_blocked() {
        let mut board = Board::new();
        let white_pawn = Pawn::new(Direction::North);
        let start_pos = Position::from((5, 4));
        board.set(
            &Position::from((4, 4)),
            Some(Piece::new(
                PieceType::Pawn(Pawn::new(Direction::South)),
                Color::Black,
            )),
        );

        let moves = white_pawn.get_moves(Color::White, true, start_pos, &board);

        assert!(
            moves.is_empty(),
            "Pawn should not be able to move forward if blocked"
        );
    }

    #[test]
    fn test_pawn_capture_move() {
        let mut board = Board::new();
        let white_pawn = Pawn::new(Direction::North);
        let start_pos = Position::from((4, 4));
        board.set(
            &Position::from((3, 3)),
            Some(Piece::new(
                PieceType::Pawn(Pawn::new(Direction::South)),
                Color::Black,
            )),
        );
        board.set(
            &Position::from((3, 5)),
            Some(Piece::new(
                PieceType::Pawn(Pawn::new(Direction::South)),
                Color::Black,
            )),
        );

        let moves = white_pawn.get_moves(Color::White, true, start_pos, &board);

        assert_eq!(moves.len(), 3);
        assert!(moves.contains(&Move::new(
            MoveType::Normal,
            start_pos,
            Position::from((3, 3))
        )));
        assert!(moves.contains(&Move::new(
            MoveType::Normal,
            start_pos,
            Position::from((3, 5))
        )));
        assert!(moves.contains(&Move::new(
            MoveType::Normal,
            start_pos,
            Position::from((3, 4))
        )));
    }

    #[test]
    fn test_pawn_cannot_capture_own_piece() {
        let mut board = Board::new();
        let white_pawn = Pawn::new(Direction::North);
        let start_pos = Position::from((4, 4));
        board.set(
            &Position::from((3, 3)),
            Some(Piece::new(
                PieceType::Pawn(Pawn::new(Direction::North)),
                Color::White,
            )),
        );
        board.set(
            &Position::from((3, 5)),
            Some(Piece::new(
                PieceType::Pawn(Pawn::new(Direction::North)),
                Color::White,
            )),
        );

        let moves = white_pawn.get_moves(Color::White, true, start_pos, &board);

        assert!(!moves.contains(&Move::new(
            MoveType::Normal,
            start_pos,
            Position::from((3, 3))
        )));
        assert!(!moves.contains(&Move::new(
            MoveType::Normal,
            start_pos,
            Position::from((3, 5))
        )));
    }

    #[test]
    fn test_can_capture_opponent_king() {
        let mut board = Board::new();
        let white_pawn = Pawn::new(Direction::North);
        let pos = Position::from((4, 4));
        board.set(
            &Position::from((3, 3)),
            Some(Piece::new(PieceType::King(King), Color::Black)),
        );
        let can_capture_king =
            white_pawn.can_capture_opponent_king(Color::White, true, pos, &board);

        assert!(can_capture_king);
    }
}
