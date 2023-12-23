use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Write};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[derive(Eq, PartialEq, Clone)]
pub struct Vec2d<T> {
    pub(crate) inner: Vec<Vec<T>>,
}

#[derive(Copy)]
pub struct Cell<'a, T> {
    pub(crate) parent: &'a Vec2d<T>,
    pub(crate) row: usize,
    pub(crate) col: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct RowCol {
    pub row: usize,
    pub col: usize,
}

impl From<RowCol> for (usize, usize) {
    fn from(val: RowCol) -> Self {
        (val.row, val.col)
    }
}

impl From<(usize, usize)> for RowCol {
    fn from((row, col): (usize, usize)) -> Self {
        RowCol { row, col }
    }
}

/// Represents a contiguous set of cells within a specific row
/// It is guaranteed that the row value of these cells is the same
pub struct CellRowRange<'a, T> {
    parent: &'a Vec2d<T>,
    row: usize,
    first_col: usize,
    last_col: usize,
}

impl<'a, T> Clone for Cell<'a, T> {
    fn clone(&self) -> Self {
        self.parent.get_cell(self.row(), self.col()).unwrap()
    }
}

impl<'a, T> AsRef<T> for Cell<'a, T> {
    fn as_ref(&self) -> &T {
        self.value()
    }
}

impl<'a, T> Deref for Cell<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

impl<'a, T> PartialEq for Cell<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.parent, other.parent) && self.row == other.row && self.col == other.col
    }
}
impl<'a, T> Eq for Cell<'a, T> {}

impl<'a, T> Hash for Cell<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.col.hash(state);
        self.row.hash(state);
    }
}

impl<T> Vec2d<T> {
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.inner.get(row).and_then(|row| row.get(col))
    }

    pub fn get_row_col(&self, row_col: &RowCol) -> Option<&T> {
        self.get(row_col.row, row_col.col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.inner.get_mut(row).and_then(|row| row.get_mut(col))
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<Cell<T>> {
        self.get(row, col).map(|_value| Cell {
            parent: self,
            row,
            col,
        })
    }

    pub fn get_row(&self, row: usize) -> Option<&[T]> {
        self.inner.get(row).map(|row| row.as_slice())
    }

    pub fn get_col_cells(&self, col: usize) -> impl Iterator<Item = Cell<T>> {
        (0..self.inner.len()).filter_map(move |row| self.get_cell(row, col))
    }

    pub fn get_col(&self, col: usize) -> impl Iterator<Item = &T> {
        (0..self.inner.len()).filter_map(move |row| self.get(row, col))
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

    pub fn map<F: Fn(&T) -> S, S>(&self, f: F) -> Vec2d<S> {
        Vec2d {
            inner: self
                .inner
                .iter()
                .map(|row| row.iter().map(&f).collect())
                .collect(),
        }
    }

    pub fn num_rows(&self) -> usize {
        self.inner.len()
    }

    pub fn first_num_cols(&self) -> usize {
        self.inner.get(0).map(|row| row.len()).unwrap_or(0)
    }

    pub fn top_left_cell(&self) -> Option<Cell<T>> {
        self.get_cell(0, 0)
    }

    pub fn bottom_right_cell(&self) -> Option<Cell<T>> {
        let last_row = self.inner.len() - 1;
        let Some(last_col) = self.inner.get(last_row).map(|row| row.len() - 1) else {
            return None;
        };

        self.get_cell(last_row, last_col)
    }
}

impl<T: Copy> Vec2d<T> {
    pub fn with_shape_and_value(rows: usize, cols: usize, value: T) -> Vec2d<T> {
        Vec2d {
            inner: (0..rows).map(|_| vec![value; cols]).collect(),
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

impl<T> Vec2d<Option<T>> {
    pub fn flat_get(&self, row: usize, col: usize) -> Option<&T> {
        self.get(row, col).and_then(|val| val.as_ref())
    }

    pub fn flat_get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.get_mut(row, col).and_then(|val| val.as_mut())
    }
}

impl<T: Debug> Debug for Vec2d<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.inner {
            row.fmt(f)?;
            f.write_char('\n')?;
        }
        Ok(())
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

    pub fn coords(&self) -> RowCol {
        RowCol {
            row: self.row,
            col: self.col,
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn get_top(&self) -> Option<Cell<T>> {
        self.get_diff(-1, 0)
    }

    pub fn get_below(&self) -> Option<Cell<T>> {
        self.get_diff(1, 0)
    }

    pub fn get_left(&self) -> Option<Cell<T>> {
        self.get_diff(0, -1)
    }

    pub fn get_right(&self) -> Option<Cell<T>> {
        self.get_diff(0, 1)
    }

    pub fn cardinal_neighbors(&self) -> impl Iterator<Item = Cell<T>> {
        [
            self.get_top(),
            self.get_right(),
            self.get_below(),
            self.get_left(),
        ]
        .into_iter()
        .flatten()
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

    /// find the first cell in the row of the current cell
    pub fn first_cell_in_row(&self) -> Cell<T> {
        Cell {
            parent: self.parent,
            row: self.row,
            col: 0,
        }
    }

    /// return the last cell in the current row
    pub fn last_cell_in_row(&self) -> Cell<T> {
        Cell {
            parent: self.parent,
            row: self.row,
            col: self.parent.inner[self.row].len() - 1,
        }
    }

    /// returns the row of the current cell
    pub fn get_row(&self) -> &[T] {
        self.parent.inner[self.row].as_slice()
    }

    pub fn find_first_before<P>(&self, predicate: P) -> Option<Cell<T>>
    where
        P: Fn(&T) -> bool,
    {
        let row_vec = &self.parent.inner[self.row];
        for (i, value) in row_vec.iter().enumerate().take(self.col).rev() {
            if predicate(value) {
                return self.parent.get_cell(self.row, i);
            }
        }
        None
    }

    pub fn find_first_after<P>(&self, predicate: P) -> Option<Cell<T>>
    where
        P: Fn(&T) -> bool,
    {
        let row_vec = &self.parent.inner[self.row];
        for (i, value) in row_vec.iter().enumerate().skip(self.col + 1) {
            if predicate(value) {
                return self.parent.get_cell(self.row, i);
            }
        }
        None
    }

    /// Find the longest contiguous range of neighbors of this cell in the same row satisfying
    /// the given predicate.
    pub fn find_contiguous_satisfying<P>(&self, predicate: P) -> CellRowRange<T>
    where
        P: Fn(&T) -> bool,
    {
        let first_cell = self.find_first_before(|val| !predicate(val));
        let first_cell = first_cell
            .as_ref()
            .and_then(|cell| cell.next_col())
            .unwrap_or_else(|| self.first_cell_in_row());

        let last_cell = self.find_first_after(|val| !predicate(val));
        let last_cell = last_cell
            .as_ref()
            .and_then(|cell| cell.prev_col())
            .unwrap_or_else(|| self.last_cell_in_row());

        CellRowRange {
            parent: self.parent,
            row: self.row,
            first_col: first_cell.col,
            last_col: last_cell.col,
        }
    }

    pub fn prev_col(&self) -> Option<Cell<T>> {
        if self.col == 0 {
            None
        } else {
            self.parent.get_cell(self.row, self.col - 1)
        }
    }
    pub fn next_col(&self) -> Option<Cell<T>> {
        self.parent.get_cell(self.row, self.col + 1)
    }

    /// Modify the row and column by the specified value and return the cell at the coordiante,
    /// if it exists
    pub fn get_diff(&self, row: isize, col: isize) -> Option<Cell<T>> {
        self.row
            .checked_add_signed(row)
            .and_then(|row| self.col.checked_add_signed(col).map(|col| (row, col)))
            .and_then(|(row, col)| self.parent.get_cell(row, col))
    }
}

impl<'a, T> PartialOrd for Cell<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if !std::ptr::eq(self.parent, other.parent) {
            return None;
        }
        match self.row.cmp(&other.row) {
            Ordering::Equal => Some(self.col.cmp(&other.col)),
            ordering => Some(ordering),
        }
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

impl<'a, T> PartialEq for CellRowRange<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.parent, other.parent)
            && self.row == other.row
            && self.first_col == other.first_col
            && self.last_col == other.last_col
    }
}

impl<'a, T> Eq for CellRowRange<'a, T> {}

impl<'a, T> PartialOrd for CellRowRange<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.first()
            .partial_cmp(&other.first())
            .or_else(|| self.last().partial_cmp(&other.last()))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn next(&self, RowCol { row, col }: RowCol) -> Option<RowCol> {
        match self {
            Direction::Up => row.checked_sub(1).map(|row| RowCol { row, col }),
            Direction::Down => Some(RowCol { row: row + 1, col }),
            Direction::Left => col.checked_sub(1).map(|col| RowCol { row, col }),
            Direction::Right => Some(RowCol { row, col: col + 1 }),
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
