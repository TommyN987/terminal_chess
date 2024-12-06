use serde::{Deserialize, Serialize};

use crate::{
    board::{Board, Direction, Position},
    game::Color,
    moves::{Move, MoveType, Moveable},
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Queen;

impl Moveable for Queen {
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

    fn get_moves(
        &self,
        _color: Color,
        _has_moved: bool,
        from: Position,
        board: &Board,
    ) -> Vec<Move> {
        self.reachable_positions_in_many_directions(from, board)
            .into_iter()
            .map(|pos| Move::new(MoveType::Normal, from, pos))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::pieces::{Piece, PieceType};

    use super::*;

    #[test]
    fn test_queen_unblocked_moves() {
        // Arrange

        let mut board = Board::default();
        let queen_position = Position::from((4, 4));
        let queen = Queen;

        board.set(
            &queen_position,
            Some(Piece::new(PieceType::Queen(Queen), Color::White)),
        );

        // Act
        let moves = queen.get_moves(Color::White, true, queen_position, &board);

        // Assert
        let expected_positions = vec![
            Position::from((4, 0)),
            Position::from((4, 1)),
            Position::from((4, 2)),
            Position::from((4, 3)),
            Position::from((4, 5)),
            Position::from((4, 6)),
            Position::from((4, 7)),
            Position::from((0, 4)),
            Position::from((1, 4)),
            Position::from((2, 4)),
            Position::from((3, 4)),
            Position::from((5, 4)),
            Position::from((6, 4)),
            Position::from((7, 4)),
            Position::from((5, 5)),
            Position::from((6, 6)),
            Position::from((7, 7)),
            Position::from((3, 3)),
            Position::from((2, 2)),
            Position::from((1, 1)),
            Position::from((0, 0)),
            Position::from((5, 3)),
            Position::from((6, 2)),
            Position::from((7, 1)),
            Position::from((3, 5)),
            Position::from((2, 6)),
            Position::from((1, 7)),
        ];

        let result_positions: Vec<_> = moves.iter().map(|m| m.to).collect();

        assert_eq!(result_positions.len(), expected_positions.len());

        for pos in expected_positions {
            assert!(
                result_positions.contains(&pos),
                "Expected position {:?} not found in moves",
                pos
            );
        }
    }

    #[test]
    fn test_queen_blocked_by_same_color() {
        // Arrange
        let board = Board::new();
        let queen = Queen;
        let queen_position = Position::from((7, 7));

        // Act
        let moves = queen.get_moves(Color::White, true, queen_position, &board);

        // Assert
        assert!(moves.is_empty());
    }

    #[test]
    fn test_queen_can_capture_opponent_piece() {
        // Arrange
        let mut board = Board::default();
        let queen = Queen;
        let queen_position = Position::from((4, 4));
        let opponent_position = Position::from((4, 5));

        board.set(
            &queen_position,
            Some(Piece::new(PieceType::Queen(Queen), Color::White)),
        );

        board.set(
            &opponent_position,
            Some(Piece::new(PieceType::Queen(Queen), Color::Black)),
        );

        // Act
        let moves = queen.get_moves(Color::White, true, queen_position, &board);

        // Assert
        let result_positions: Vec<_> = moves.iter().map(|m| m.to).collect();

        for pos in result_positions.iter() {
            println!("Position: {:?}", pos);
        }

        assert!(
            result_positions.contains(&opponent_position),
            "Queen  should be able to capture at (4, 5)"
        );

        assert!(
            !result_positions.contains(&Position::from((4, 6))),
            "Queen should be blocked after capturing at (4, 5)"
        );
    }
}
