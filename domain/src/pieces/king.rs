use crate::{board::Board, direction::Direction, position::Position, Color};

use super::{
    moveable::{Move, MoveType, Moveable},
    PieceType,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct King {
    pub directions: [Direction; 8],
}

impl Default for King {
    fn default() -> Self {
        Self::new()
    }
}

impl King {
    pub fn new() -> Self {
        Self {
            directions: [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
                Direction::NorthEast,
                Direction::NorthWest,
                Direction::SouthEast,
                Direction::SouthWest,
            ],
        }
    }
}

impl Moveable for King {
    fn get_moves(
        &self,
        color: Color,
        _has_moved: bool,
        from: Position,
        board: &Board,
    ) -> Vec<super::moveable::Move> {
        self.move_positions(color, from, board)
            .into_iter()
            .map(|to| Move::new(MoveType::Normal, from, to))
            .collect()
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
            .any(|pos| match board.get(pos) {
                None => false,
                Some(piece) => piece.piece_type == PieceType::King(King::default()),
            })
    }
}

impl King {
    fn move_positions(&self, color: Color, from: Position, board: &Board) -> Vec<Position> {
        self.directions
            .iter()
            .filter_map(|dir| {
                let to = from + *dir;
                if !board.is_inside(&to) {
                    return None;
                }

                match board.get(&to) {
                    None => Some(to),
                    Some(piece) => match piece.piece_color == color {
                        true => None,
                        false => Some(to),
                    },
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::pieces::{Knight, Piece};

    use super::*;

    #[test]
    fn test_king_unblocked_moves() {
        // Arrange
        let mut board = Board::default();
        let king = King::new();
        let king_position = Position::from((4, 4));

        board.set(
            &king_position,
            Some(Piece::new(PieceType::King(King::new()), Color::White)),
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
    fn test_king_blocked_by_same_color() {
        let board = Board::new();
        let king = King::new();
        let king_position = Position::from((7, 4));

        let moves = king.get_moves(Color::White, false, king_position, &board);

        assert!(moves.is_empty());
    }

    #[test]
    fn test_king_can_capture_opponent_piece() {
        let mut board = Board::default();
        let king = King::new();
        let king_position = Position::from((4, 4));
        let opponent_position = Position::from((4, 3));

        board.set(
            &king_position,
            Some(Piece::new(PieceType::King(King::new()), Color::White)),
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
