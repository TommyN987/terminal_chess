use serde::{Deserialize, Serialize};

use crate::{
    board::{Board, Direction, Position},
    game::Color,
    moves::{Move, MoveType, Moveable},
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Knight;

impl Moveable for Knight {
    fn get_moves(
        &self,
        color: Color,
        _has_moved: bool,
        from: Position,
        board: &Board,
    ) -> Vec<Move> {
        self.move_positions(color, from, board)
            .into_iter()
            .map(|to| Move::new(MoveType::Normal, from, to))
            .collect()
    }
}

impl Knight {
    fn potential_to_positions(&self, from: Position) -> Vec<Position> {
        let mut result = vec![];
        for vertical in [Direction::North, Direction::South] {
            for horizontal in [Direction::East, Direction::West] {
                result.push(from + Position::from(vertical * 2) + horizontal);
                result.push(from + Position::from(horizontal * 2) + vertical);
            }
        }
        result
    }

    fn move_positions(&self, color: Color, from: Position, board: &Board) -> Vec<Position> {
        self.potential_to_positions(from)
            .into_iter()
            .filter(|pos| {
                board.is_inside(pos)
                    && match board[pos] {
                        None => true,
                        Some(piece) => piece.piece_color != color,
                    }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::pieces::{Pawn, Piece, PieceType};

    use super::*;

    #[test]
    fn test_knight_unblocked_moves() {
        // Arrange

        let mut board = Board::default();
        let knight_position = Position::from((4, 4));
        let knight = Knight;

        board.set(
            &knight_position,
            Some(Piece::new(PieceType::Knight(Knight), Color::White)),
        );

        // Act
        let moves = knight.get_moves(Color::White, true, knight_position, &board);

        // Assert
        let expected_position = vec![
            Position::from((3, 2)),
            Position::from((5, 2)),
            Position::from((6, 3)),
            Position::from((6, 5)),
            Position::from((5, 6)),
            Position::from((3, 6)),
            Position::from((2, 5)),
            Position::from((2, 3)),
        ];

        let result_positions: Vec<_> = moves.iter().map(|m| m.to).collect();

        assert_eq!(result_positions.len(), expected_position.len());

        for pos in expected_position {
            assert!(
                result_positions.contains(&pos),
                "Expected position {:?} not found in moves",
                pos
            );
        }
    }

    #[test]
    fn test_knight_blocked_by_same_color() {
        // Arrange
        let board = Board::new();
        let knight = Knight;

        // Act
        let result_positions: Vec<_> = knight
            .get_moves(Color::White, false, Position::from((7, 1)), &board)
            .into_iter()
            .map(|m| m.to)
            .collect();

        // Assert
        assert!(!result_positions.contains(&Position::from((6, 3))));
    }

    #[test]
    fn test_knight_can_capture_opponent_piece() {
        // Arrange
        let mut board = Board::new();
        let knight = Knight;
        let knight_position = Position::from((7, 1));
        let opponent_position = Position::from((6, 3));

        board.set(
            &opponent_position,
            Some(Piece::new(
                PieceType::Pawn(Pawn::new(Direction::South)),
                Color::Black,
            )),
        );

        // Act
        let result_positions: Vec<_> = knight
            .get_moves(Color::White, false, knight_position, &board)
            .into_iter()
            .map(|m| m.to)
            .collect();

        assert!(
            result_positions.contains(&opponent_position),
            "Knight should be able to capture at (6, 3)"
        );
    }
}
