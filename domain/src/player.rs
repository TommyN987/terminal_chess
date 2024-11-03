use derive_new::new;

use crate::Color;

#[derive(Debug, Clone, PartialEq, new)]
pub struct Player {
    pub color: Color,
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
