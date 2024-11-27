use crate::{
    board::{Board, Direction, Position},
    moves::{Move, MoveType},
    pieces::{Piece, PieceKind, PieceType},
};

use super::{Color, Player};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FenString(String);

impl FenString {
    pub(super) fn derive(board: &Board, player: &Player) -> Self {
        let inner = format!(
            "{} {} {} {}",
            Self::piece_placement(board),
            Self::current_player(player),
            Self::castling_rights(board),
            Self::en_passant(board, player.color)
        );

        Self(inner)
    }

    pub(super) fn inner(self) -> String {
        self.0
    }
}

impl FenString {
    fn piece_placement(board: &Board) -> String {
        let mut piece_placement = String::new();
        for (row, _) in board.fields.iter().enumerate() {
            piece_placement.push_str(&format!("{}/", Self::row_data(board, row)));
        }
        piece_placement.trim_end_matches('/').to_string()
    }

    fn row_data(board: &Board, row: usize) -> String {
        let (mut row_string, empty_squares_count) = board.fields[row].iter().fold(
            (String::new(), 0u8),
            |(mut row_string, mut empty_squares_count), square| {
                match square {
                    None => empty_squares_count += 1,
                    Some(piece) => {
                        if empty_squares_count > 0 {
                            row_string.push_str(&empty_squares_count.to_string());
                            empty_squares_count = 0;
                        }
                        row_string.push_str(&Self::piece_char(piece));
                    }
                }
                (row_string, empty_squares_count)
            },
        );

        if empty_squares_count > 0 {
            row_string.push_str(&empty_squares_count.to_string());
        }

        row_string
    }

    fn piece_char(piece: &Piece) -> String {
        match piece.piece_color {
            Color::White => PieceKind::from(piece.piece_type).to_string().to_uppercase(),
            Color::Black => PieceKind::from(piece.piece_type).to_string(),
        }
    }

    fn current_player(player: &Player) -> char {
        match player.color {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }

    fn castling_rights(board: &Board) -> String {
        let mut castling_rights = String::new();
        let can_white_short_castle = Self::can_short_castle(board, Color::White);
        let can_white_long_castle = Self::can_long_castle(board, Color::White);
        let can_black_short_castle = Self::can_short_castle(board, Color::Black);
        let can_black_long_castle = Self::can_long_castle(board, Color::Black);

        if !(can_white_short_castle
            || can_white_long_castle
            || can_black_short_castle
            || can_black_long_castle)
        {
            castling_rights.push('-');
            return castling_rights.to_string();
        }

        if can_white_short_castle {
            castling_rights.push('K');
        }

        if can_white_long_castle {
            castling_rights.push('Q');
        }

        if can_black_short_castle {
            castling_rights.push('k');
        }

        if can_black_long_castle {
            castling_rights.push('q');
        }

        castling_rights
    }

    fn en_passant(board: &Board, player: Color) -> String {
        let mut en_passant = String::new();
        if !Self::can_capture_en_passant(board, player) {
            en_passant.push('-');
            return en_passant;
        }

        if let Some(pos) = board.get_en_passant_square(&player) {
            let rank = (8 - pos.row).to_string();
            let file = ('a' as u8 + pos.column as u8) as char;
            en_passant.push(file);
            en_passant.push_str(&rank);
        }
        en_passant
    }

    fn can_capture_en_passant(board: &Board, player: Color) -> bool {
        if let Some(en_passant_pos) = board.get_en_passant_square(&player.opponent()) {
            let pawn_positions = match player {
                Color::White => [
                    en_passant_pos + Direction::SouthWest,
                    en_passant_pos + Direction::SouthEast,
                ],
                Color::Black => [
                    en_passant_pos + Direction::NorthWest,
                    en_passant_pos + Direction::NorthEast,
                ],
            };

            pawn_positions
                .iter()
                .filter(|pos| board.is_inside(pos))
                .any(|pos| {
                    if let Some(piece) = board[pos] {
                        if piece.piece_color == player
                            && matches!(piece.piece_type, PieceType::Pawn(_))
                        {
                            let en_passant_move =
                                Move::new(MoveType::EnPassant, *pos, en_passant_pos);
                            return en_passant_move.is_legal(board);
                        }
                    }
                    false
                })
        } else {
            false
        }
    }

    fn can_short_castle(board: &Board, player: Color) -> bool {
        match player {
            Color::White => Self::are_king_and_rook_unmoved(
                board,
                Position::from((7, 4)),
                Position::from((7, 7)),
            ),
            Color::Black => Self::are_king_and_rook_unmoved(
                board,
                Position::from((0, 4)),
                Position::from((0, 7)),
            ),
        }
    }

    fn can_long_castle(board: &Board, player: Color) -> bool {
        match player {
            Color::White => Self::are_king_and_rook_unmoved(
                board,
                Position::from((7, 4)),
                Position::from((7, 0)),
            ),
            Color::Black => Self::are_king_and_rook_unmoved(
                board,
                Position::from((0, 4)),
                Position::from((0, 0)),
            ),
        }
    }

    fn are_king_and_rook_unmoved(board: &Board, king_pos: Position, rook_pos: Position) -> bool {
        let king = match board[&king_pos] {
            None => return false,
            Some(king) => king,
        };

        let rook = match board[&rook_pos] {
            None => return false,
            Some(rook) => rook,
        };

        !king.has_moved && !rook.has_moved
    }
}
