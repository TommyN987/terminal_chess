use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, new)]
pub struct Player {
    pub color: Color,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            color: Color::White,
        }
    }
}

impl Player {
    pub fn opponent(&self) -> Self {
        match self.color {
            Color::White => Self {
                color: Color::Black,
            },
            Color::Black => Self {
                color: Color::White,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
