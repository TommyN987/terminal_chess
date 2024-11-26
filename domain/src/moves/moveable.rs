use crate::{
    board::{Board, Direction, Position},
    pieces::PieceKind,
    Color,
};

use super::Move;

pub(crate) trait Moveable {
    const DIRECTIONS: &'static [Direction] = &[];

    fn get_moves(&self, color: Color, has_moved: bool, from: Position, board: &Board) -> Vec<Move>;

    fn reachable_positions_in_direction(
        &self,
        from: Position,
        dir: &Direction,
        board: &Board,
    ) -> Vec<Position> {
        let mut result = vec![];
        if let Some(current_piece) = board.get(&from) {
            let mut pos = from + *dir;
            while board.is_inside(&pos) {
                match board.get(&pos) {
                    None => result.push(pos),
                    Some(piece) if current_piece.piece_color != piece.piece_color => {
                        result.push(pos);
                        break;
                    }
                    Some(_) => break,
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
    ) -> Vec<Position> {
        Self::DIRECTIONS
            .iter()
            .flat_map(|dir| self.reachable_positions_in_direction(from, dir, board))
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
            .any(|m| board[&m.to].map_or(false, |piece| piece.piece_type.into() == PieceKind::King))
    }
}
