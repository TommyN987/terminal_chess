use std::ops::{Index, IndexMut};

use crate::pieces::Piece;

use super::{Board, Position};

impl Index<&Position> for Board {
    type Output = Option<Piece>;

    fn index(&self, index: &Position) -> &Self::Output {
        &self.fields[index.row as usize][index.column as usize]
    }
}

impl Index<(u8, u8)> for Board {
    type Output = Option<Piece>;

    fn index(&self, (row, col): (u8, u8)) -> &Self::Output {
        &self.fields[row as usize][col as usize]
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Option<Piece>;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.fields[row][col]
    }
}

impl IndexMut<&Position> for Board {
    fn index_mut(&mut self, index: &Position) -> &mut Option<Piece> {
        &mut self.fields[index.row as usize][index.column as usize]
    }
}

impl IndexMut<(u8, u8)> for Board {
    fn index_mut(&mut self, (row, col): (u8, u8)) -> &mut Option<Piece> {
        &mut self.fields[row as usize][col as usize]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Option<Piece> {
        &mut self.fields[row][col]
    }
}
