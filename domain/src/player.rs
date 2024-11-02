use crate::Color;

pub struct Player {
    color: Color,
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
