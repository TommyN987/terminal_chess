use crate::{
    board::{Board, Direction, Position},
    game::{Color, Player},
    moves::{Move, MoveType, Moveable},
};

use super::PieceType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct King;

impl Moveable for King {
    const DIRECTIONS: &'static [Direction] = &[
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
        Direction::NorthEast,
        Direction::NorthWest,
        Direction::SouthEast,
        Direction::SouthWest,
    ];

    fn get_moves(&self, color: Color, has_moved: bool, from: Position, board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = self
            .move_positions(color, from, board)
            .into_iter()
            .map(|to| Move::new(MoveType::Normal, from, to))
            .collect();

        if self.can_short_castle(has_moved, &from, board) {
            moves.push(Move::new(
                MoveType::ShortCastle,
                from,
                Position::from((from.row, 6)),
            ));
        }

        if self.can_long_castle(has_moved, &from, board) {
            moves.push(Move::new(
                MoveType::LongCastle,
                from,
                Position::from((from.row, 2)),
            ));
        }

        moves
    }

    fn can_capture_opponent_king(
        &self,
        color: Color,
        _has_moved: bool,
        from: Position,
        board: &Board,
    ) -> bool {
        self.move_positions(color, from, board)
            .iter()
            .any(|pos| match board[pos] {
                None => false,
                Some(piece) => piece.piece_type == PieceType::King(King),
            })
    }
}

impl King {
    fn move_positions(&self, color: Color, from: Position, board: &Board) -> Vec<Position> {
        Self::DIRECTIONS
            .iter()
            .filter_map(|dir| {
                let to = from + *dir;
                if !board.is_inside(&to) {
                    return None;
                }

                match board[&to] {
                    None => Some(to),
                    Some(piece) => match piece.piece_color == color {
                        true => None,
                        false => Some(to),
                    },
                }
            })
            .collect()
    }

    fn can_short_castle(&self, has_moved: bool, from: &Position, board: &Board) -> bool {
        let Position { row, column: _ } = *from;
        let rook_position = Position::from((row, 7));
        let between_positions = (5..=6)
            .map(|col| Position::from((row, col as i8)))
            .collect();

        let player = if row == 7 {
            Player::new(Color::White)
        } else {
            Player::new(Color::Black)
        };

        self.is_rook_unmoved(&rook_position, board)
            && self.are_castling_squares_empty(between_positions, board)
            && !board.is_in_check(player)
            && !has_moved
    }

    fn can_long_castle(&self, has_moved: bool, from: &Position, board: &Board) -> bool {
        let Position { row, column: _ } = *from;
        let rook_position = Position::from((row, 0));
        let between_positions = (1..=3)
            .map(|col| Position::from((row, col as i8)))
            .collect();

        let player = if row == 7 {
            Player::new(Color::White)
        } else {
            Player::new(Color::Black)
        };

        self.is_rook_unmoved(&rook_position, board)
            && self.are_castling_squares_empty(between_positions, board)
            && !board.is_in_check(player)
            && !has_moved
    }

    fn is_rook_unmoved(&self, position: &Position, board: &Board) -> bool {
        match board[position] {
            None => false,
            Some(piece) => !piece.has_moved,
        }
    }

    fn are_castling_squares_empty(&self, positions: Vec<Position>, board: &Board) -> bool {
        positions.iter().all(|pos| board[pos].is_none())
    }
}

#[cfg(test)]
mod tests {
    use crate::pieces::{Bishop, Knight, Piece, Rook};

    use super::*;

    #[test]
    fn test_king_unblocked_moves() {
        // Arrange
        let mut board = Board::default();
        let king = King;
        let king_position = Position::from((4, 4));

        board.set(
            &king_position,
            Some(Piece::new(PieceType::King(King), Color::White)),
        );

        // Act
        let moves = king.get_moves(Color::White, true, king_position, &board);

        // Assert
        let expected_positions = vec![
            Position::from((4, 3)),
            Position::from((4, 5)),
            Position::from((3, 3)),
            Position::from((3, 4)),
            Position::from((3, 5)),
            Position::from((5, 3)),
            Position::from((5, 4)),
            Position::from((5, 5)),
        ];

        let result_positions: Vec<_> = moves.iter().map(|m| m.to).collect();

        assert_eq!(result_positions.len(), expected_positions.len());

        for pos in expected_positions {
            assert!(result_positions.contains(&pos));
        }
    }

    #[test]
    fn test_short_castle() {
        let mut board = Board::default();
        let king = King;
        let king_position = Position::from((7, 4));
        let rook_position = Position::from((7, 7));
        let mut our_rook = Piece::new(PieceType::Rook(Rook), Color::White);
        let opponent_bishop = Piece::new(PieceType::Bishop(Bishop), Color::Black);

        board.set(
            &king_position,
            Some(Piece::new(PieceType::King(King), Color::White)),
        );

        board.set(&rook_position, Some(our_rook));

        let moves = king.get_moves(Color::White, false, king_position, &board);

        assert!(moves.contains(&Move::new(
            MoveType::ShortCastle,
            king_position,
            Position::from((7, 6))
        )));

        // Place the bishop so it checks the king
        board.set(&Position::from((3, 0)), Some(opponent_bishop));

        let moves = king.get_moves(Color::White, false, king_position, &board);

        assert!(
            !moves.contains(&Move::new(
                MoveType::ShortCastle,
                king_position,
                Position::from((7, 6))
            )),
            "Castling not allowed when king is in check."
        );

        // Remove bishop
        board.set(&Position::from((3, 0)), None);

        // Make the king having been moved
        let moves = king.get_moves(Color::White, true, king_position, &board);

        assert!(
            !moves.contains(&Move::new(
                MoveType::ShortCastle,
                king_position,
                Position::from((7, 6))
            )),
            "Castling not allowed when king has already moved."
        );

        // Make our rook having been moved
        our_rook.has_moved = true;
        board.set(&rook_position, Some(our_rook));
        let moves = king.get_moves(Color::White, false, king_position, &board);

        assert!(
            !moves.contains(&Move::new(
                MoveType::ShortCastle,
                king_position,
                Position::from((7, 6))
            )),
            "Castling not allowed when rook has already moved."
        );

        // Block square between king and rook
        board = Board::new();
        let moves = king.get_moves(Color::White, false, king_position, &board);

        assert!(
            !moves.contains(&Move::new(
                MoveType::ShortCastle,
                king_position,
                Position::from((7, 6))
            )),
            "Castling not allowed when squares between king and rook are blocked."
        );
    }

    #[test]
    fn test_king_blocked_by_same_color() {
        let board = Board::new();
        let king = King;
        let king_position = Position::from((7, 4));

        let moves = king.get_moves(Color::White, false, king_position, &board);

        assert!(moves.is_empty());
    }

    #[test]
    fn test_king_can_capture_opponent_piece() {
        let mut board = Board::default();
        let king = King;
        let king_position = Position::from((4, 4));
        let opponent_position = Position::from((4, 3));

        board.set(
            &king_position,
            Some(Piece::new(PieceType::King(King), Color::White)),
        );

        board.set(
            &opponent_position,
            Some(Piece::new(PieceType::Knight(Knight), Color::Black)),
        );

        // Act
        let moves = king.get_moves(Color::White, true, king_position, &board);

        // Assert
        let expected_positions = vec![
            Position::from((4, 3)),
            Position::from((4, 5)),
            Position::from((3, 3)),
            Position::from((3, 4)),
            Position::from((3, 5)),
            Position::from((5, 3)),
            Position::from((5, 4)),
            Position::from((5, 5)),
        ];

        let result_positions: Vec<_> = moves.iter().map(|m| m.to).collect();

        assert_eq!(result_positions.len(), expected_positions.len());

        for pos in expected_positions {
            assert!(result_positions.contains(&pos));
        }
    }
}
