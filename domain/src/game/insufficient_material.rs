use crate::pieces::{Bishop, Knight, PieceType};

use super::PieceCounter;

#[derive(Debug, Default)]
pub(super) struct InsufficientMaterial(bool);

impl InsufficientMaterial {
    pub(super) fn derive(piece_counter: &PieceCounter) -> Self {
        Self::default()
            .king_vs_king(piece_counter)
            .king_knight_vs_king(piece_counter)
            .king_bishop_vs_king(piece_counter)
    }

    pub(super) fn inner(self) -> bool {
        self.0
    }

    fn king_vs_king(mut self, piece_counter: &PieceCounter) -> Self {
        if piece_counter.get_total() == 2 {
            self.0 = true;
        }

        self
    }

    fn king_knight_vs_king(mut self, piece_counter: &PieceCounter) -> Self {
        if piece_counter.get_total() == 3
            && (piece_counter.get_white(&PieceType::Knight(Knight)) == 1
                || piece_counter.get_black(&PieceType::Knight(Knight)) == 1)
        {
            self.0 = true;
        }

        self
    }

    fn king_bishop_vs_king(mut self, piece_counter: &PieceCounter) -> Self {
        if piece_counter.get_total() == 3
            && (piece_counter.get_white(&PieceType::Bishop(Bishop)) == 1
                || piece_counter.get_black(&PieceType::Bishop(Bishop)) == 1)
        {
            self.0 = true;
        }

        self
    }
}
