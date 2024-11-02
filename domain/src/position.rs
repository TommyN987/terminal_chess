use std::ops::Add;

use derive_new::new;

use crate::{direction::Direction, Color};

#[derive(Debug, Copy, Clone, PartialEq, Hash, new)]
pub struct Position {
    pub row: i8,
    pub column: i8,
}

impl Position {
    pub fn square_color(&self) -> Color {
        if self.row + self.column % 2 == 0 {
            return Color::White;
        }
        Color::Black
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        Position::new(self.row + rhs.row_delta(), self.column + rhs.column_delta())
    }
}
