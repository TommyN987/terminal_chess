use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    pub fn row_delta(&self) -> i8 {
        match self {
            Self::North | Self::NorthEast | Self::NorthWest => -1,
            Self::South | Self::SouthEast | Self::SouthWest => 1,
            Self::East | Self::West => 0,
        }
    }

    pub fn column_delta(&self) -> i8 {
        match self {
            Self::North | Self::South => 0,
            Self::East | Self::NorthEast | Self::SouthEast => 1,
            Self::West | Self::NorthWest | Self::SouthWest => -1,
        }
    }
}

impl Add for Direction {
    type Output = (i8, i8);

    fn add(self, rhs: Self) -> Self::Output {
        (
            self.row_delta() + rhs.row_delta(),
            self.column_delta() + rhs.column_delta(),
        )
    }
}

impl Mul<i8> for Direction {
    type Output = (i8, i8);

    fn mul(self, rhs: i8) -> Self::Output {
        (self.row_delta() * rhs, self.column_delta() * rhs)
    }
}
