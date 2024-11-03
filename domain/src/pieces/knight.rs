use crate::{board::Board, direction::Direction, position::Position, Color};

use super::moveable::{Move, MoveType, Moveable};

#[derive(Debug, Clone, Copy, PartialEq)]
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
                    && match board.get(pos) {
                        None => true,
                        Some(piece) => piece.piece_color != color,
                    }
            })
            .collect()
    }
}
