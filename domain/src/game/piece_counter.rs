use crate::pieces::PieceType;

use super::Color;

pub struct PieceCounter {
    white: [u8; 6],
    black: [u8; 6],
    total: u8,
}

impl Default for PieceCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl PieceCounter {
    pub fn new() -> Self {
        Self {
            white: [0; 6],
            black: [0; 6],
            total: 0,
        }
    }

    pub fn increment(&mut self, color: &Color, piece: &PieceType) {
        match color {
            Color::White => self.white[piece.as_index()] += 1,
            Color::Black => self.black[piece.as_index()] += 1,
        };
        self.total += 1;
    }

    pub fn get_white(&self, piece: &PieceType) -> u8 {
        self.white[piece.as_index()]
    }

    pub fn get_black(&self, piece: &PieceType) -> u8 {
        self.black[piece.as_index()]
    }

    pub fn get_total(&self) -> u8 {
        self.total
    }
}
