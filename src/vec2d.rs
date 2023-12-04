use std::fmt::{Display, Write};

#[derive(Debug, Eq, PartialEq)]
pub struct Vec2d<T> {
    pub(crate) inner: Vec<Vec<T>>,
}

#[derive(Debug, Copy)]
pub struct Cell<'a, T> {
    pub(crate) parent: &'a Vec2d<T>,
    pub(crate) row: usize,
    pub(crate) col: usize,
}

/// Represents a contiguous set of cells within a specific row
/// It is guaranteed that the row value of these cells is the same
#[derive(Eq, PartialEq, Debug)]
pub struct CellRowRange<'a, T> {
    parent: &'a Vec2d<T>,
    row: usize,
    first_col: usize,
    last_col: usize,
}

impl<'a, T> Clone for Cell<'a, T> {
    fn clone(&self) -> Self {
        Cell {
            parent: self.parent,
            row: self.row,
            col: self.col,
        }
    }
}

impl<'a, T> PartialEq for Cell<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.parent, other.parent) && self.row == other.row && self.col == other.col
    }
}
impl<'a, T> Eq for Cell<'a, T> {}

impl<T> Vec2d<T> {
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.inner.get(row).and_then(|row| row.get(col))
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<Cell<T>> {
        self.get(row, col).map(|_value| Cell {
            parent: self,
            row,
            col,
        })
    }

    pub fn cells(&self) -> impl Iterator<Item = Cell<T>> {
        self.inner
            .iter()
            .enumerate()
            .flat_map(move |(row, row_val)| {
                row_val.iter().enumerate().map(move |(col, _value)| Cell {
                    parent: self,
                    row,
                    col,
                })
            })
    }
    pub fn get_range(&self, row: usize, first_col: usize, last_col: usize) -> CellRowRange<T> {
        CellRowRange {
            parent: self,
            row,
            first_col,
            last_col,
        }
    }
}

impl Vec2d<char> {
    pub fn from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Vec2d<char> {
        Vec2d {
            inner: lines.map(|line| line.as_ref().chars().collect()).collect(),
        }
    }
}

impl<'a> Display for CellRowRange<'a, char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.as_slice() {
            f.write_char(*c)?
        }
        Ok(())
    }
}

impl<T> Cell<'_, T> {
    pub fn value(&self) -> &T {
        &self.parent.inner[self.row][self.col]
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Cell<T>> {
        let row = self.row as i32;
        let col = self.col as i32;
        let neighbor_indices = [
            (row - 1, col - 1),
            (row, col - 1),
            (row + 1, col - 1),
            (row - 1, col),
            (row + 1, col),
            (row - 1, col + 1),
            (row, col + 1),
            (row + 1, col + 1),
        ];

        neighbor_indices
            .into_iter()
            .filter_map(|(row, col)| {
                let row = usize::try_from(row);
                let col = usize::try_from(col);

                match (row, col) {
                    (Ok(row), Ok(col)) => Some((row, col)),
                    _ => None,
                }
            })
            .filter_map(|(row, col)| self.parent.get_cell(row, col))
    }

    pub fn next_col(&self) -> Option<Cell<T>> {
        self.parent.get_cell(self.row, self.col + 1)
    }
}

impl<'a, T> CellRowRange<'a, T> {
    pub fn cells(&self) -> impl Iterator<Item = Cell<T>> {
        (self.first_col..=self.last_col).map(|col| {
            self.parent
                .get_cell(self.row, col)
                .ok_or_else(|| anyhow::Error::msg("Invalid row range"))
                .unwrap()
        })
    }

    pub fn first(&self) -> Cell<T> {
        Cell {
            parent: self.parent,
            row: self.row,
            col: self.first_col,
        }
    }

    pub fn last(&self) -> Cell<T> {
        Cell {
            parent: self.parent,
            row: self.row,
            col: self.last_col,
        }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.parent.inner[self.row].as_slice()[self.first_col..=self.last_col]
    }
}
