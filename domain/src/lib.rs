pub mod board;
pub mod game;
pub mod insufficient_material;
pub mod moves;
pub mod pieces;
pub mod player;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opponent(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
