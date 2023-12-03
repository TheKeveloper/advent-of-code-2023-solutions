use std::fmt::Display;

use advent_of_code_2023_solutions::Solution;

fn main() {
    Day3::default_print_solution()
}

struct Day3 {}

impl Solution for Day3 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> u32 {
        let matrix: Vec2d<char> = Vec2d::from_lines(lines);
        unimplemented!()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Vec2d<T> {
    inner: Vec<Vec<T>>,
}

#[derive(Debug, Copy)]
struct Cell<'a, T> {
    parent: &'a Vec2d<T>,
    row: usize,
    col: usize,
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

/// Represents a contiguous set of cells within a specific row
/// It is guaranteed that the row value of these cells is the same
#[derive(Eq, PartialEq, Debug)]
struct CellRowRange<'a, T> {
    parent: &'a Vec2d<T>,
    row: usize,
    first_col: usize,
    last_col: usize,
}

impl<T> Vec2d<T> {
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.inner.get(row).and_then(|row| row.get(col))
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<Cell<T>> {
        self.get(row, col).map(|value| Cell {
            parent: &self,
            row,
            col,
        })
    }

    pub fn cells(&self) -> impl Iterator<Item = Cell<T>> {
        self.inner
            .iter()
            .enumerate()
            .flat_map(move |(row, row_val)| {
                let row = row.clone();
                row_val.iter().enumerate().map(move |(col, value)| Cell {
                    parent: &self,
                    row,
                    col,
                })
            })
    }
}

impl Vec2d<char> {
    pub fn from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Vec2d<char> {
        Vec2d {
            inner: lines.map(|line| line.as_ref().chars().collect()).collect(),
        }
    }
}

impl<T> Cell<'_, T> {
    pub fn value(&self) -> &T {
        &self.parent.inner[self.row][self.col]
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Cell<T>> {
        let neighbor_indices = [
            (self.row - 1, self.col - 1),
            (self.row, self.col - 1),
            (self.row + 1, self.col - 1),
            (self.row - 1, self.col),
            (self.row + 1, self.col),
            (self.row - 1, self.col + 1),
            (self.row, self.col + 1),
            (self.row + 1, self.col + 1),
        ];

        neighbor_indices
            .into_iter()
            .filter_map(|(row, col)| self.parent.get_cell(row, col))
    }

    pub fn next_col(&self) -> Option<Cell<T>> {
        self.parent.get_cell(self.row, self.col + 1)
    }
}

fn is_symbol(c: &char) -> bool {
    !c.is_alphabetic() && *c != '.'
}

impl<'a, T> CellRowRange<'a, T> {
    pub fn cells(&self) -> impl Iterator<Item = Cell<T>> {
        (self.first_col..=self.last_col).into_iter().map(|col| {
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

impl<'a> Display for CellRowRange<'a, char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parent = self.parent;
        let row = self.row;
        write!(f, "{}", self.as_slice().iter().collect::<String>())
    }
}
