use std::iter::zip;
use std::str::FromStr;

use itertools::Itertools;

use crate::common::Solution;
use crate::vec2d::Vec2d;

pub enum Day13 {}

impl Solution for Day13 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let combined = lines.map(|s| s.as_ref().to_string()).join("\n");

        combined
            .split("\n\n")
            .map(|s| s.parse::<Pattern>().unwrap().get_reflection_summary())
            .sum::<usize>()
            .to_string()
    }
}

struct Pattern {
    tiles: Vec2d<Tile>,
}

impl Pattern {
    pub fn get_reflection_summary(&self) -> usize {
        if let Some(row) = self.get_reflecting_row() {
            row * 100
        } else {
            self.get_reflecting_col().unwrap()
        }
    }

    fn get_reflecting_col(&self) -> Option<usize> {
        let num_cols = self.get_num_cols();
        for col in 0..(num_cols - 1) {
            for margin in 1..num_cols {
                if let Some(mut it) = self.reflect_right_of_col(col, margin) {
                    if !it.all(|(a, b)| a.eq(&b)) {
                        break;
                    }
                } else {
                    return Some(col + 1);
                }
            }
        }
        None
    }

    fn get_reflecting_row(&self) -> Option<usize> {
        let num_rows = self.tiles.num_rows();
        for row in 0..(num_rows - 1) {
            for margin in 1..num_rows {
                if let Some(mut it) = self.reflect_below_row(row, margin) {
                    if !it.all(|(a, b)| a.eq(&b)) {
                        break;
                    }
                } else {
                    return Some(row + 1);
                }
            }
        }
        None
    }

    fn reflect_right_of_col(
        &self,
        col: usize,
        margin: usize,
    ) -> Option<impl Iterator<Item = (Tile, Tile)> + '_> {
        let Some(left_col) = (col + 1).checked_sub(margin) else {
            return None;
        };
        let right_col = col + margin;
        if right_col >= self.get_num_cols() {
            return None;
        }

        Some(zip(
            self.tiles.get_col(left_col).map(|t| *t),
            self.tiles.get_col(right_col).map(|t| *t),
        ))
    }

    fn reflect_below_row(
        &self,
        row: usize,
        margin: usize,
    ) -> Option<impl Iterator<Item = (Tile, Tile)> + '_> {
        let Some(bottom_row) = (row + 1).checked_sub(margin) else {
            return None;
        };
        let top_row = row + margin;
        if top_row >= self.tiles.inner.len() {
            return None;
        }

        Some(zip(
            self.tiles.get_row(bottom_row).unwrap().iter().map(|t| *t),
            self.tiles.get_row(top_row).unwrap().iter().map(|t| *t),
        ))
    }

    fn get_num_cols(&self) -> usize {
        self.tiles.first_num_cols()
    }
}

impl FromStr for Pattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matrix = Vec2d::from_lines(s.lines());
        Ok(Pattern {
            tiles: matrix.map(|c| Tile::from(*c)),
        })
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Tile {
    Ash,
    Rock,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Ash,
            '#' => Tile::Rock,
            _ => panic!("Received invalid value: {}", value),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day13::Day13;

    const EXAMPLE_INPUT: &'static str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
    #[test]
    fn test_example() {
        assert_eq!(Day13::solve(EXAMPLE_INPUT.lines()), "405")
    }
}
