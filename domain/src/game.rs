use crate::{
    board::Board,
    pieces::{moveable::Move, Moveable, Piece},
    player::Player,
    position::Position,
};

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub board: Board,
    pub current_player: Player,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: Player::default(),
        }
    }

    pub fn legal_moves_for_piece(&self, from: Position) -> Option<(Piece, Vec<Move>)> {
        match self.board.get(&from) {
            None => None,
            Some(piece) => {
                if piece.piece_color == self.current_player.color {
                    Some((
                        piece,
                        piece
                            .get_moves(piece.piece_color, piece.has_moved, from, &self.board)
                            .into_iter()
                            .filter(|m| piece.is_legal(*m, from, &self.board))
                            .collect(),
                    ))
                } else {
                    None
                }
            }
        }
    }

    pub fn make_move(&mut self, piece: Piece, m: Move) {
        piece.execute(m, &mut self.board);
        self.current_player = self.current_player.opponent();
    }
}
