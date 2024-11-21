use crate::{
    board::Board,
    pieces::{moveable::Move, Moveable, Piece, PieceType},
    player::Player,
    position::Position,
};

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub board: Board,
    pub current_player: Player,
    pub promotion_move: Option<(Move, PieceType)>,
    pub result: Option<GameResult>,
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
            promotion_move: None,
            result: None,
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
                            .filter(|m| m.is_legal(&self.board))
                            .collect(),
                    ))
                } else {
                    None
                }
            }
        }
    }

    pub fn make_move(&mut self, m: Move) {
        match self.promotion_move {
            None => m.execute(&mut self.board, None),
            Some((m, piece_type)) => {
                let promotion_piece = Piece::new(piece_type, self.current_player.color);
                m.execute(&mut self.board, Some(promotion_piece));
                self.promotion_move = None;
            }
        };
        self.current_player = self.current_player.opponent();
        self.check_for_game_over();
    }

    pub fn is_game_over(&self) -> bool {
        self.result.is_some()
    }

    fn all_legal_moves_for_player(&self, player: &Player) -> Vec<Move> {
        self.board
            .piece_positions_for_player(player)
            .iter()
            .filter_map(|pos| {
                self.board.get(pos).map(|piece| {
                    piece
                        .get_moves(piece.piece_color, piece.has_moved, *pos, &self.board)
                        .into_iter()
                        .filter(|m| m.is_legal(&self.board))
                        .collect::<Vec<Move>>()
                })
            })
            .flatten()
            .collect()
    }

    fn check_for_game_over(&mut self) {
        if self
            .all_legal_moves_for_player(&self.current_player)
            .is_empty()
        {
            if self.board.is_in_check(self.current_player) {
                self.result = Some(GameResult::win(self.current_player.opponent()))
            } else {
                self.result = Some(GameResult::draw(EndReason::Stalemate));
            }
        }

        if self.board.insufficient_material() {
            self.result = Some(GameResult::draw(EndReason::InsufficientMaterial));
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EndReason {
    Checkmate,
    Stalemate,
    FiftyMoveRule,
    InsufficientMaterial,
    ThreefoldRepetition,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameResult {
    pub winner: Option<Player>,
    pub end_reason: EndReason,
}

impl GameResult {
    pub fn win(winner: Player) -> Self {
        Self {
            winner: Some(winner),
            end_reason: EndReason::Checkmate,
        }
    }

    pub fn draw(end_reason: EndReason) -> Self {
        Self {
            winner: None,
            end_reason,
        }
    }
}
