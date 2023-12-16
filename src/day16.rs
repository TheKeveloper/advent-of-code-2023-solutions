use std::collections::HashSet;

use crate::common::Solution;
use crate::vec2d::{Direction, RowCol, Vec2d};

pub enum Day16 {}

impl Solution for Day16 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let mut grid: Grid = Vec2d::from_lines(lines)
            .map(|c| Tile::from(*c))
            .map(|tile| (*tile).into())
            .into();

        grid.travel_and_mark(
            grid.cells.top_left_cell().unwrap().coords(),
            Direction::Right,
        );

        grid.count_energized().to_string()
    }
}

pub enum Day16P2 {}

impl Solution for Day16P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        todo!()
    }
}

struct Grid {
    cells: Vec2d<GridCell>,
}

impl Grid {
    pub fn travel_and_mark(&mut self, start: RowCol, direction: Direction) {
        let Some(cur_cell) = self.cells.get_mut(start.row, start.col) else {
            return;
        };
        if cur_cell.energized && cur_cell.traveled_directions.contains(&direction) {
            return;
        } else {
            cur_cell.energized = true;
            cur_cell.traveled_directions.insert(direction);
        }

        let tile = cur_cell.tile;

        match (tile, direction) {
            (Tile::Empty, _)
            | (Tile::VerticalSplit, Direction::Up | Direction::Down)
            | (Tile::HorizontalSplit, Direction::Left | Direction::Right) => {
                if let Some(next) = direction.next(start) {
                    self.travel_and_mark(next, direction)
                }
            }
            (Tile::ForwardMirror, Direction::Right) | (Tile::BackwardMirror, Direction::Left) => {
                if let Some(next) = Direction::Up.next(start) {
                    self.travel_and_mark(next, Direction::Up)
                }
            }
            (Tile::BackwardMirror, Direction::Right) | (Tile::ForwardMirror, Direction::Left) => {
                if let Some(next) = Direction::Down.next(start) {
                    self.travel_and_mark(next, Direction::Down)
                }
            }
            (Tile::ForwardMirror, Direction::Down) | (Tile::BackwardMirror, Direction::Up) => {
                if let Some(next) = Direction::Left.next(start) {
                    self.travel_and_mark(next, Direction::Left)
                }
            }
            (Tile::ForwardMirror, Direction::Up) | (Tile::BackwardMirror, Direction::Down) => {
                if let Some(next) = Direction::Right.next(start) {
                    self.travel_and_mark(next, Direction::Right)
                }
            }
            (Tile::VerticalSplit, Direction::Left | Direction::Right) => {
                if let Some(next) = Direction::Up.next(start) {
                    self.travel_and_mark(next, Direction::Up)
                }
                if let Some(next) = Direction::Down.next(start) {
                    self.travel_and_mark(next, Direction::Down)
                }
            }
            (Tile::HorizontalSplit, Direction::Up | Direction::Down) => {
                if let Some(next) = Direction::Left.next(start) {
                    self.travel_and_mark(next, Direction::Left);
                }
                if let Some(next) = Direction::Right.next(start) {
                    self.travel_and_mark(next, Direction::Right)
                }
            }
        }
    }
    pub fn count_energized(&self) -> usize {
        self.cells.cells().filter(|cell| cell.energized).count()
    }
}

impl From<Vec2d<GridCell>> for Grid {
    fn from(value: Vec2d<GridCell>) -> Self {
        Grid { cells: value }
    }
}

struct GridCell {
    tile: Tile,
    energized: bool,
    traveled_directions: HashSet<Direction>,
}

impl From<Tile> for GridCell {
    fn from(tile: Tile) -> Self {
        GridCell {
            tile,
            energized: false,
            traveled_directions: HashSet::new(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    // '.'
    Empty,
    // '|'
    VerticalSplit,
    // '-'
    HorizontalSplit,
    // '/'
    ForwardMirror,
    // '\'
    BackwardMirror,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '|' => Tile::VerticalSplit,
            '-' => Tile::HorizontalSplit,
            '/' => Tile::ForwardMirror,
            '\\' => Tile::BackwardMirror,
            _ => panic!("Invalid char received: {}", value),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day16::Day16;

    const EXAMPLE_INPUT: &'static str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_example() {
        assert_eq!(Day16::solve(EXAMPLE_INPUT.lines()), "46")
    }
}
