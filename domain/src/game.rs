use derive_new::new;

use crate::{
    board::Board,
    pieces::{moveable::Move, Moveable, Piece},
    player::Player,
    position::Position,
};

#[derive(Debug, Clone, PartialEq, new)]
pub struct Game {
    pub board: Board,
    pub current_player: Player,
}

impl Game {
    pub fn legal_moves_for_piece(&self, from: Position) -> Option<(Piece, Vec<Move>)> {
        match self.board.get(&from) {
            None => None,
            Some(piece) => {
                if piece.piece_color == self.current_player.color {
                    Some((
                        piece,
                        piece.get_moves(piece.piece_color, piece.has_moved, from, &self.board),
                    ))
                } else {
                    None
                }
            }
        }
    }

    pub fn make_move(&mut self, piece: Piece, m: Move) {
        piece.execute(m, &mut self.board);
    }
}
