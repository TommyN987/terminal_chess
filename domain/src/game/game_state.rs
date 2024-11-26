use crate::{
    board::{Board, Position},
    moves::{moveable::Moveable, Move, MoveRecord},
    pieces::{Piece, PieceKind},
};

use super::{InsufficientMaterial, Player};

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub board: Board,
    pub current_player: Player,
    pub move_history: Vec<MoveRecord>,
    pub non_capture_or_pawn_move_counter: u8,
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
            move_history: vec![],
            non_capture_or_pawn_move_counter: 0,
            result: None,
        }
    }

    pub fn legal_moves_for_piece(&self, from: Position) -> Option<(Piece, Vec<Move>)> {
        match self.board[&from] {
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
        let move_record = m.execute(&mut self.board);
        if move_record.piece_captured.is_some() || move_record.piece_moved == PieceKind::Pawn {
            self.non_capture_or_pawn_move_counter = 0;
        } else {
            self.non_capture_or_pawn_move_counter += 1;
        }
        self.move_history.push(move_record);
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
                self.board[pos].map(|piece| {
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

        if self.insufficient_material() {
            self.result = Some(GameResult::draw(EndReason::InsufficientMaterial));
        }

        if self.fifty_move_rule() {
            self.result = Some(GameResult::draw(EndReason::FiftyMoveRule));
        }
    }

    fn insufficient_material(&self) -> bool {
        let piece_counter = self.board.count_pieces();
        *InsufficientMaterial::derive(&piece_counter)
    }

    fn fifty_move_rule(&self) -> bool {
        self.non_capture_or_pawn_move_counter >= 100
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
