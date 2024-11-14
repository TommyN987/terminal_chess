use derive_new::new;

use crate::{board::Board, direction::Direction, player::Player, position::Position, Color};

use super::{King, PieceType};

pub trait Moveable {
    fn get_moves(&self, color: Color, has_moved: bool, from: Position, board: &Board) -> Vec<Move>;

    fn execute(self, legal_move: Move, board: &mut Board)
    where
        Self: Sized,
    {
        let piece = board.get(&legal_move.from);
        board.set(&legal_move.to, piece);
        board.set(&legal_move.from, None);
        if let Some(mut piece) = piece {
            piece.has_moved = true;
        }
    }

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

    fn is_legal(&self, m: Move, from: Position, board: &Board) -> bool {
        if let Some(piece) = board.get(&from) {
            let player = Player::new(piece.piece_color).opponent();
            let mut cloned_board = board.clone();
            piece.execute(m, &mut cloned_board);
            return cloned_board.is_in_check(player);
        }
        false
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
