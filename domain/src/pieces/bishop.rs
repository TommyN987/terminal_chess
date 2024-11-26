use crate::{
    board::{Board, Direction, Position},
    game::Color,
    moves::{Move, MoveType, Moveable},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bishop;

impl Moveable for Bishop {
    const DIRECTIONS: &'static [Direction] = &[
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
    use crate::pieces::{Pawn, Piece, PieceType};

    use super::*;

    #[test]
    fn test_bishop_unblocked_moves() {
        let mut board = Board::default();
        let bishop_position = Position::from((4, 4));

        // Place a White Bishop on the board
        board.set(
            &bishop_position,
            Some(Piece::new(PieceType::Bishop(Bishop), Color::White)),
        );

        // Generate moves
        let bishop = Bishop;
        let moves = bishop.get_moves(Color::White, false, bishop_position, &board);

        // Bishop should have moves in all four diagonal directions from (4, 4)
        let expected_positions = vec![
            Position::from((5, 5)),
            Position::from((6, 6)),
            Position::from((7, 7)), // SouthEast
            Position::from((3, 3)),
            Position::from((2, 2)),
            Position::from((1, 1)),
            Position::from((0, 0)), // NorthWest
            Position::from((5, 3)),
            Position::from((6, 2)),
            Position::from((7, 1)), // SouthWest
            Position::from((3, 5)),
            Position::from((2, 6)),
            Position::from((1, 7)), // NorthEast
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
    fn test_bishop_blocked_by_same_color() {
        let mut board = Board::new();
        let bishop_position = Position::from((4, 4));
        let blocker_position = Position::from((5, 5));

        // Place a White Bishop and a White blocker piece
        board.set(
            &bishop_position,
            Some(Piece::new(PieceType::Bishop(Bishop), Color::White)),
        );
        board.set(
            &blocker_position,
            Some(Piece::new(
                PieceType::Pawn(Pawn::new(Direction::North)),
                Color::White,
            )),
        );

        // Generate moves
        let bishop = Bishop;
        let moves = bishop.get_moves(Color::White, false, bishop_position, &board);

        // Bishop should not be able to move past the blocker at (5, 5)
        let result_positions: Vec<_> = moves.iter().map(|m| m.to).collect();
        assert!(
            !result_positions.contains(&Position::from((6, 6))),
            "Bishop should be blocked at (5, 5)"
        );
        assert!(
            result_positions.contains(&Position::from((3, 3))),
            "Bishop should be able to move to (3, 3)"
        );
    }

    #[test]
    fn test_bishop_can_capture_opponent_piece() {
        let mut board = Board::new();
        let bishop_position = Position::from((4, 4));
        let opponent_position = Position::from((5, 5));

        // Place a White Bishop and a Black Pawn as an opponent
        board.set(
            &bishop_position,
            Some(Piece::new(PieceType::Bishop(Bishop), Color::White)),
        );
        board.set(
            &opponent_position,
            Some(Piece::new(
                PieceType::Pawn(Pawn::new(Direction::South)),
                Color::Black,
            )),
        );

        // Generate moves
        let bishop = Bishop;
        let moves = bishop.get_moves(Color::White, false, bishop_position, &board);

        // Bishop should be able to capture the opponent at (5, 5) but not move past it
        let result_positions: Vec<_> = moves.iter().map(|m| m.to).collect();
        assert!(
            result_positions.contains(&opponent_position),
            "Bishop should be able to capture at (5, 5)"
        );
        assert!(
            !result_positions.contains(&Position::from((6, 6))),
            "Bishop should be blocked after capturing at (5, 5)"
        );
    }

    #[test]
    fn test_bishop_edge_of_board_moves() {
        let mut board = Board::default();
        let bishop_position = Position::from((7, 7));

        // Place a White Bishop
        board.set(
            &bishop_position,
            Some(Piece::new(PieceType::Bishop(Bishop), Color::White)),
        );

        // Generate moves
        let bishop = Bishop;
        let moves = bishop.get_moves(Color::White, false, bishop_position, &board);

        // Bishop should only be able to move inwards along the board from the corner
        let expected_positions = vec![
            Position::from((6, 6)),
            Position::from((5, 5)),
            Position::from((4, 4)),
            Position::from((3, 3)),
            Position::from((2, 2)),
            Position::from((1, 1)),
            Position::from((0, 0)),
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
}
