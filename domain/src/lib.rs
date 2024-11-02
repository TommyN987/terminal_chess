pub mod board;
pub mod direction;
pub mod pieces;
pub mod player;
pub mod position;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}
