use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::{Bishop, King, Knight, Pawn, Queen, Rook};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Display for PieceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pawn => write!(f, "p"),
            Self::Knight => write!(f, "n"),
            Self::Bishop => write!(f, "b"),
            Self::Rook => write!(f, "r"),
            Self::Queen => write!(f, "q"),
            Self::King => write!(f, "k"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum PromotionPiece {
    Knight,
    Bishop,
    Rook,
    #[default]
    Queen,
}

impl Display for PromotionPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Queen => 'Q',
                Self::Rook => 'R',
                Self::Bishop => 'B',
                Self::Knight => 'N',
            }
        )
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PieceType {
    Pawn(Pawn),
    Bishop(Bishop),
    Knight(Knight),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

impl PieceType {
    pub fn as_index(&self) -> usize {
        match self {
            PieceType::Pawn(_) => 0,
            PieceType::Knight(_) => 1,
            PieceType::Bishop(_) => 2,
            PieceType::Rook(_) => 3,
            PieceType::Queen(_) => 4,
            PieceType::King(_) => 5,
        }
    }
}

impl From<&PieceType> for PieceKind {
    fn from(value: &PieceType) -> Self {
        match value {
            PieceType::Pawn(_) => Self::Pawn,
            PieceType::Knight(_) => Self::Knight,
            PieceType::Bishop(_) => Self::Bishop,
            PieceType::Rook(_) => Self::Rook,
            PieceType::Queen(_) => Self::Queen,
            PieceType::King(_) => Self::King,
        }
    }
}

impl From<&PromotionPiece> for PieceType {
    fn from(promotion_piece: &PromotionPiece) -> Self {
        match promotion_piece {
            PromotionPiece::Knight => Self::Knight(Knight),
            PromotionPiece::Bishop => Self::Bishop(Bishop),
            PromotionPiece::Rook => Self::Rook(Rook),
            PromotionPiece::Queen => Self::Queen(Queen),
        }
    }
}

impl From<PieceType> for PromotionPiece {
    fn from(value: PieceType) -> Self {
        match value {
            PieceType::Knight(_) => PromotionPiece::Knight,
            PieceType::Bishop(_) => PromotionPiece::Bishop,
            PieceType::Rook(_) => PromotionPiece::Rook,
            _ => PromotionPiece::Queen,
        }
    }
}

impl PartialEq for PieceType {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (PieceType::King(_), PieceType::King(_))
                | (PieceType::Rook(_), PieceType::Rook(_))
                | (PieceType::Bishop(_), PieceType::Bishop(_))
                | (PieceType::Pawn(_), PieceType::Pawn(_))
                | (PieceType::Queen(_), PieceType::Queen(_))
                | (PieceType::Knight(_), PieceType::Knight(_))
        )
    }
}
