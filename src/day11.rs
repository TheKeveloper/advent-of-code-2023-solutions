use std::collections::HashSet;

use itertools::Itertools;

use crate::common::Solution;
use crate::vec2d::{Cell, Vec2d};

pub enum Day11 {}
impl Solution for Day11 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let matrix = Vec2d::from_lines(lines);
        let matrix = matrix.map(|c| Point::from(*c));
        let image: Image = matrix.into();

        image
            .get_galaxies()
            .combinations(2)
            .map(|val| {
                let [a, b] = val.as_slice() else { panic!() };

                image.dist(a.coords(), b.coords(), 2)
            })
            .sum::<usize>()
            .to_string()
    }
}

pub enum Day11P2 {}
impl Solution for Day11P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let matrix = Vec2d::from_lines(lines);
        let matrix = matrix.map(|c| Point::from(*c));
        let image: Image = matrix.into();

        image
            .get_galaxies()
            .combinations(2)
            .map(|val| {
                let [a, b] = val.as_slice() else { panic!() };

                image.dist(a.coords(), b.coords(), 1_000_000)
            })
            .sum::<usize>()
            .to_string()
    }
}

struct Image {
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
    points: Vec2d<Point>,
}

impl Image {
    pub fn get_galaxies(&self) -> impl Iterator<Item = Cell<Point>> {
        self.points.cells().filter(|cell| cell.value().is_galaxy())
    }

    pub fn dist(
        &self,
        (a_row, a_col): (usize, usize),
        (b_row, b_col): (usize, usize),
        expansion: usize,
    ) -> usize {
        let min_row = std::cmp::min(a_row, b_row);
        let max_row = std::cmp::max(a_row, b_row);

        let min_col = std::cmp::min(a_col, b_col);
        let max_col = std::cmp::max(a_col, b_col);

        let row_dist: usize = (min_row..max_row)
            .map(|row| {
                if self.empty_rows.contains(&row) {
                    expansion
                } else {
                    1
                }
            })
            .sum();

        let col_dist: usize = (min_col..max_col)
            .map(|col| {
                if self.empty_cols.contains(&col) {
                    expansion
                } else {
                    1
                }
            })
            .sum();

        row_dist + col_dist
    }
}

impl From<Vec2d<Point>> for Image {
    fn from(value: Vec2d<Point>) -> Self {
        let mut empty_rows = HashSet::new();
        let mut empty_cols = HashSet::new();

        let col_count = value.get_row(0).unwrap().len();
        (0..col_count)
            .filter(|col| {
                value
                    .get_col_cells(*col)
                    .all(|cell| !cell.value().is_galaxy())
            })
            .for_each(|col| {
                empty_cols.insert(col);
            });

        value
            .inner
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().all(|cell| !cell.is_galaxy()))
            .for_each(|(index, _)| {
                empty_rows.insert(index);
            });

        Image {
            empty_rows,
            empty_cols,
            points: value,
        }
    }
}

enum Point {
    Space,
    Galaxy,
}

impl Point {
    pub fn is_galaxy(&self) -> bool {
        matches!(&self, Point::Galaxy)
    }
}

impl From<char> for Point {
    fn from(value: char) -> Self {
        match value {
            '#' => Point::Galaxy,
            _ => Point::Space,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day11::Day11;

    const INPUT: &'static str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
    #[test]
    fn test_example() {
        assert_eq!(Day11::solve(INPUT.lines()), "374")
    }
}
