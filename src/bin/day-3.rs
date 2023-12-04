#![allow(dead_code)]

use std::fmt::{Display, Write};

use advent_of_code_2023_solutions::Solution;

fn main() {
    Day3::default_print_solution()
}

struct Day3 {}

impl Solution for Day3 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> u32 {
        let matrix: Vec2d<char> = Vec2d::from_lines(lines);
        get_values(&matrix).sum()
    }
}

fn get_values(matrix: &Vec2d<char>) -> impl Iterator<Item = u32> + '_ {
    get_numeric_ranges(matrix)
        .into_iter()
        .filter(|range| borders_symbol(range))
        .map(|range| range.to_string())
        .map(|s| s.parse::<u32>().unwrap())
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

fn is_symbol(c: &char) -> bool {
    !c.is_numeric() && *c != '.'
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

impl<'a> Display for CellRowRange<'a, char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.as_slice() {
            f.write_char(*c)?
        }
        Ok(())
    }
}

fn borders_symbol(range: &CellRowRange<char>) -> bool {
    for cell in range.cells() {
        for neighbor in cell.neighbors() {
            if is_symbol(neighbor.value()) {
                return true;
            }
        }
    }
    false
}

fn get_numeric_ranges(matrix: &Vec2d<char>) -> Vec<CellRowRange<'_, char>> {
    let mut ranges = Vec::new();

    for (row, row_vec) in matrix.inner.iter().enumerate() {
        let mut start: Option<usize> = None;
        for (col, val) in row_vec.iter().enumerate() {
            if val.is_ascii_digit() {
                match start {
                    None => start = Some(col),
                    Some(_) => {}
                }
            } else {
                match start {
                    None => {}
                    Some(first_col) => {
                        ranges.push(CellRowRange {
                            parent: matrix,
                            row,
                            first_col,
                            last_col: col - 1,
                        });
                        start = None
                    }
                }
            }
        }
        match start {
            None => {}
            Some(first_col) => ranges.push(CellRowRange {
                parent: matrix,
                row,
                first_col,
                last_col: row_vec.len() - 1,
            }),
        }
    }
    ranges
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_example() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(Day3::solve(input.lines()), 4361)
    }

    #[test]
    fn test_get_values() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let matrix: Vec2d<char> = Vec2d::from_lines(input.lines());
        let values: Vec<u32> = get_values(&matrix).collect();
        assert_eq!(values, vec![467, 35, 633, 617, 592, 755, 664, 598]);
    }
}
