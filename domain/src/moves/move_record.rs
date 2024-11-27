use std::fmt::Display;

use crate::{board::Position, pieces::PieceKind};

use super::{Move, MoveType};

#[derive(Debug, Clone, PartialEq)]
pub struct MoveRecord {
    pub mv: Move,
    pub piece_moved: PieceKind,
    pub piece_captured: Option<PieceKind>,
    pub is_check: bool,
}

impl Display for MoveRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.mv.move_type {
            MoveType::ShortCastle => write!(f, "0-0"),
            MoveType::LongCastle => write!(f, "0-0-0"),
            MoveType::Promotion(piece) => {
                write!(f, "{}={}", self.pgn_preprocess(), piece.to_string())
            }
            _ => write!(f, "{}", self.pgn_preprocess()),
        }
    }
}

impl MoveRecord {
    fn pgn_preprocess(&self) -> String {
        let piece_char = match self.piece_moved {
            PieceKind::Pawn => "",
            _ => &self.piece_moved.to_string().to_uppercase(),
        };

        let Position { row, column } = self.mv.to;
        let rank = 8 - row;
        let file = ('a' as u8 + column as u8) as char;

        format!(
            "{}{}{}{}",
            piece_char,
            file,
            rank,
            if self.is_check { "+" } else { "" }
        )
    }
}
