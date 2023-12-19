use std::ops::Sub;
use std::str::FromStr;

use itertools::Itertools;

use crate::common::Solution;
use crate::vec2d::Direction;

pub enum Day18 {}

impl Solution for Day18 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let plan = Plan::from_lines(lines);
        plan.get_area().to_string()
    }
}

pub enum Day18P2 {}
impl Solution for Day18P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let plan = Plan::from_lines(lines).part2();
        plan.get_area().to_string()
    }
}

struct Plan {
    instructions: Vec<Instruction>,
}

impl Plan {
    pub fn from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Self {
        Plan {
            instructions: lines.map(|s| s.as_ref().parse().unwrap()).collect(),
        }
    }

    pub fn part2(self) -> Self {
        Plan {
            instructions: self
                .instructions
                .into_iter()
                .map(|instruction| instruction.part2())
                .collect(),
        }
    }

    pub fn get_area(&self) -> usize {
        // https://en.wikipedia.org/wiki/Shoelace_formula#Triangle_formula
        let interior = (self
            .get_path()
            .iter()
            .map(|(_, pos)| pos)
            .chain([SignedRowCol { row: 0, col: 0 }].iter())
            .tuple_windows()
            .map(|(a, b)| ((a.col * b.row) as f64) - ((a.row * b.col) as f64))
            .sum::<f64>()
            .abs()
            / 2.0) as usize;
        let boundary: usize = self.instructions.iter().map(|x| x.count).sum();
        // Pick's theorem: https://en.wikipedia.org/wiki/Pick%27s_theorem
        interior + (boundary / 2) + 1
    }

    pub fn get_path(&self) -> Vec<(&Instruction, SignedRowCol)> {
        let mut path = Vec::new();
        let mut cur: SignedRowCol = SignedRowCol { row: 0, col: 0 };

        for instruction in &self.instructions {
            path.push((instruction, cur));
            cur = cur.move_in_direction(&instruction.direction, instruction.count as isize);
        }

        path
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct SignedRowCol {
    row: isize,
    col: isize,
}

impl SignedRowCol {
    pub fn move_in_direction(&self, direction: &Direction, steps: isize) -> SignedRowCol {
        let row = self.row;
        let col = self.col;
        match direction {
            Direction::Up => SignedRowCol {
                row: row.sub(steps),
                col,
            },
            Direction::Down => SignedRowCol {
                row: row + steps,
                col,
            },
            Direction::Left => SignedRowCol {
                row,
                col: col.sub(steps),
            },
            Direction::Right => SignedRowCol {
                row,
                col: col + steps,
            },
        }
    }
}

struct Instruction {
    direction: Direction,
    count: usize,
    color: String,
}

impl Instruction {
    pub fn part2(&self) -> Self {
        let color = self.color.as_str();
        let (count, direction) = color.split_at(5);

        let direction = match direction.trim() {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!("Received invalid direction string: {}", direction),
        };

        Instruction {
            count: usize::from_str_radix(count, 16).unwrap(),
            direction,
            color: self.color.clone(),
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (direction, rest) = line
            .split_once(' ')
            .ok_or_else(|| anyhow::Error::msg("Could not split line").context(line.to_string()))?;
        let (count, rest) = rest
            .split_once(' ')
            .ok_or_else(|| anyhow::Error::msg("Could not split rest").context(rest.to_string()))?;
        let color = rest
            .strip_prefix("(#")
            .and_then(|s| s.strip_suffix(')'))
            .ok_or_else(|| {
                anyhow::Error::msg("Could not strip prefix and suffix from color")
                    .context(rest.to_string())
            })?;

        Ok(Instruction {
            direction: direction_from_string(direction)?,
            count: count.parse()?,
            color: color.to_string(),
        })
    }
}

fn direction_from_string(s: &str) -> Result<Direction, anyhow::Error> {
    match s.trim() {
        "U" => Ok(Direction::Up),
        "D" => Ok(Direction::Down),
        "R" => Ok(Direction::Right),
        "L" => Ok(Direction::Left),
        _ => {
            Err(anyhow::Error::msg("Could not parse direction from string").context(s.to_string()))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day18::{Day18, Day18P2, Instruction};
    use crate::vec2d::Direction;

    const EXAMPLE_INPUT: &str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    #[test]
    fn test_example() {
        assert_eq!(Day18::solve(EXAMPLE_INPUT.lines()), "62")
    }

    #[test]
    fn test_parse() {
        let instruction: Instruction = "R 6 (#70c710)".parse().unwrap();
        assert_eq!(instruction.direction, Direction::Right);
        assert_eq!(instruction.count, 6);
        assert_eq!(instruction.color, "70c710");
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(Day18P2::solve(EXAMPLE_INPUT.lines()), "952408144115")
    }
}
