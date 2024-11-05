use derive_new::new;

use crate::Color;

#[derive(Debug, Clone, Copy, PartialEq, new)]
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
