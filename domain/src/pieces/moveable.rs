use derive_new::new;

use crate::{board::Board, direction::Direction, position::Position};

pub trait Moveable {
    fn get_moves(&self, from: Position, board: &Board) -> Vec<Move>;

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
                        } else {
                            break;
                        }
                    }
                }
                pos = pos + *dir;
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
}

#[derive(Debug, Clone)]
pub enum MoveType {
    Normal,
    ShortCastle,
    LongCastle,
    DoublePawn,
    EnPassant,
    Promotion,
}

#[derive(Debug, Clone, new)]
pub struct Move {
    pub move_type: MoveType,
    pub from: Position,
    pub to: Position,
}
