use std::ops::{Add, AddAssign};

use derive_new::new;

use crate::Color;

use super::Direction;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, new)]
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

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            column: self.column + rhs.column,
        }
    }
}

impl From<(i32, i32)> for Position {
    fn from(value: (i32, i32)) -> Self {
        Self {
            row: value.0 as i8,
            column: value.1 as i8,
        }
    }
}

impl From<(i8, i8)> for Position {
    fn from(value: (i8, i8)) -> Self {
        Self {
            row: value.0,
            column: value.1,
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            row: value.0 as i8,
            column: value.1 as i8,
        }
    }
}
