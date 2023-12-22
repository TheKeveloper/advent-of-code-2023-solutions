#![allow(dead_code)]

use std::collections::HashSet;
use std::str::FromStr;

use itertools::Itertools;

use crate::common::Solution;

pub enum Day22 {}

impl Solution for Day22 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let mut snapshot = Snapshot::from_lines(lines);
        snapshot.drop_bricks();
        (snapshot.bricks.len() - snapshot.count_sole_supporting()).to_string()
    }
}

pub enum Day22P2 {}
impl Solution for Day22P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let mut snapshot = Snapshot::from_lines(lines);
        snapshot.drop_bricks();
        snapshot.get_maximal_disintegration_count().to_string()
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Snapshot {
    bricks: Vec<Brick>,
}

impl Snapshot {
    pub fn from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Snapshot {
        Snapshot {
            bricks: lines.map(|line| line.as_ref().parse().unwrap()).collect(),
        }
    }

    fn get_supporters(&self) -> Vec<HashSet<usize>> {
        let mut supporters: Vec<HashSet<usize>> = vec![HashSet::new(); self.bricks.len()];

        for (i, brick) in self.bricks.iter().enumerate() {
            for (j, other) in self.bricks.iter().enumerate() {
                if i == j {
                    continue;
                }

                if other.supporting(brick) {
                    supporters[i].insert(j);
                }
            }
        }

        supporters
    }

    pub fn count_sole_supporting(&self) -> usize {
        self.get_supporters()
            .iter()
            .filter(|val| val.len() == 1)
            .map(|vec| vec.iter().next().unwrap())
            .unique()
            .count()
    }

    pub fn get_maximal_disintegration_count(&self) -> usize {
        let all_supporters = self.get_supporters();
        let mut total: usize = 0;
        for i in 0..self.bricks.len() {
            let mut temp_supporters = all_supporters.clone();
            let mut removed: HashSet<usize> = HashSet::new();
            removed.insert(i);
            loop {
                let mut new_removed = false;
                for j in 0..self.bricks.len() {
                    let supporters = temp_supporters.get_mut(j).unwrap();
                    let mut any_removed = false;
                    for supporter in supporters.clone().iter() {
                        if removed.contains(supporter) {
                            supporters.remove(supporter);
                            any_removed = true;
                        }
                    }
                    if supporters.is_empty() && any_removed {
                        removed.insert(j);
                        new_removed = true;
                    }
                }
                if !new_removed {
                    break;
                }
            }
            total += removed.len() - 1;
        }

        total
    }

    fn drop_bricks(&mut self) {
        self.bricks.sort_by_key(|brick| brick.highest_point());
        for i in 0..self.bricks.len() {
            let brick = &self.bricks[i];
            let lowest_point = brick.lowest_point();
            let lowest_possible = self
                .bricks
                .iter()
                .filter(|&other| other.intersects(brick))
                .filter(|&other| other.ne(brick))
                .map(|other| other.highest_point())
                .filter(|&highest_point| highest_point < lowest_point)
                .max()
                .unwrap_or(0)
                + 1;

            let drop_amount = lowest_point - lowest_possible;

            self.bricks.get_mut(i).unwrap().drop_by(drop_amount);
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Brick {
    first: Position,
    second: Position,
}

impl Brick {
    pub fn bottom(&self) -> i64 {
        std::cmp::min(self.first.z, self.second.z)
    }

    pub fn drop_by(&mut self, amount: i64) {
        self.first.z -= amount;
        self.second.z -= amount;
    }

    pub fn highest_point(&self) -> i64 {
        self.get_min_max_on_axis(&Axis::Z).1
    }

    pub fn lowest_point(&self) -> i64 {
        self.get_min_max_on_axis(&Axis::Z).0
    }

    pub fn supporting(&self, other: &Self) -> bool {
        self.intersects_naive(other) && self.highest_point() == other.lowest_point() - 1
    }

    pub fn intersects_naive(&self, other: &Self) -> bool {
        let self_cubes: HashSet<_> = self
            .cubes()
            .into_iter()
            .map(|position| position.get_2d())
            .collect();
        let other_cubes: HashSet<_> = other
            .cubes()
            .into_iter()
            .map(|position| position.get_2d())
            .collect();

        self_cubes.intersection(&other_cubes).count() > 0
    }

    pub fn intersects(&self, other: &Self) -> bool {
        // we can take advantage of the fact that none of the bricks are diagonal to simplify

        let self_axis = self.get_axis().unwrap_or(Axis::Z);
        let other_axis = other.get_axis().unwrap_or(Axis::Z);

        match (self_axis, other_axis) {
            (Axis::Z, Axis::Z) => self.first.x == other.first.x && self.first.y == other.first.y,
            (_, Axis::Z) => self.contains_point_ignore_height(&other.first),
            (Axis::Z, _) => other.contains_point_ignore_height(&self.first),
            (self_axis, other_axis) if self_axis == other_axis => {
                let other_planar_axis = self_axis.other_planar().unwrap();
                if self.first.get(&other_planar_axis) != other.first.get(&other_planar_axis) {
                    return false;
                }

                let (self_min, self_max) = self.get_min_max_on_axis(&self_axis);
                let (other_min, other_max) = other.get_min_max_on_axis(&other_axis);

                self_min <= other_max && self_max >= other_min
            }
            (self_axis, other_axis) => {
                let self_fixed = self.first.get(&other_axis);
                let other_fixed = other.first.get(&self_axis);

                let (self_min, self_max) = self.get_min_max_on_axis(&self_axis);
                let (other_min, other_max) = other.get_min_max_on_axis(&other_axis);

                self_fixed >= other_min
                    && self_fixed <= other_max
                    && other_fixed >= self_min
                    && other_fixed <= self_max
            }
        }
    }

    pub fn contains_point_ignore_height(&self, position: &Position) -> bool {
        match self.get_axis().unwrap_or(Axis::Z) {
            Axis::Z => self.first.x == position.x && self.first.y == position.y,
            axis => {
                let other_axis = axis.other_planar().unwrap();
                let (min, max) = self.get_min_max_on_axis(&axis);
                self.first.get(&other_axis) == position.get(&other_axis)
                    && position.get(&axis) <= max
                    && position.get(&axis) >= min
            }
        }
    }

    pub fn get_min_max_on_axis(&self, axis: &Axis) -> (i64, i64) {
        let first = self.first.get(axis);
        let second = self.second.get(axis);
        (std::cmp::min(first, second), std::cmp::max(first, second))
    }

    pub fn is_vertical(&self) -> bool {
        matches!(self.get_axis().unwrap_or(Axis::Z), Axis::Z)
    }

    // empty means that the brick is a single cube
    pub fn get_axis(&self) -> Option<Axis> {
        if self.first.x != self.second.x {
            Some(Axis::X)
        } else if self.first.y != self.second.y {
            Some(Axis::Y)
        } else if self.first.z != self.second.z {
            Some(Axis::Z)
        } else {
            None
        }
    }
    pub fn cubes(&self) -> Vec<Position> {
        if self.first.x != self.second.x {
            (std::cmp::min(self.first.x, self.second.x)
                ..=(std::cmp::max(self.first.x, self.second.x)))
                .map(|x| self.first.with_x(x))
                .collect()
        } else if self.first.y != self.second.y {
            (std::cmp::min(self.first.y, self.second.y)
                ..=(std::cmp::max(self.first.y, self.second.y)))
                .map(|y| self.first.with_y(y))
                .collect()
        } else if self.first.z != self.second.z {
            (std::cmp::min(self.first.z, self.second.z)
                ..=(std::cmp::max(self.first.z, self.second.z)))
                .map(|z| self.first.with_z(z))
                .collect()
        } else {
            vec![self.first]
        }
    }
}

impl FromStr for Brick {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once('~').unwrap();
        Ok(Brick {
            first: first.parse()?,
            second: second.parse()?,
        })
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn other_planar(&self) -> Option<Axis> {
        match self {
            Axis::X => Some(Axis::Y),
            Axis::Y => Some(Axis::X),
            Axis::Z => None,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Position2D {
    x: i64,
    y: i64,
}

impl Position {
    pub fn with_x(&self, new_x: i64) -> Position {
        Position { x: new_x, ..*self }
    }

    pub fn with_y(&self, new_y: i64) -> Position {
        Position { y: new_y, ..*self }
    }

    pub fn with_z(&self, new_z: i64) -> Position {
        Position { z: new_z, ..*self }
    }

    pub fn get(&self, axis: &Axis) -> i64 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub fn get_2d(&self) -> Position2D {
        Position2D {
            x: self.x,
            y: self.y,
        }
    }
}

impl FromStr for Position {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, rest) = s.split_once(',').unwrap();
        let (y, z) = rest.split_once(',').unwrap();
        Ok(Position {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use itertools::Itertools;

    use crate::common::Solution;
    use crate::day22::{Brick, Day22, Day22P2, Snapshot};

    const EXAMPLE_INPUT: &str = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    #[test]
    fn test_example() {
        assert_eq!(Day22::solve(EXAMPLE_INPUT.lines()), "5")
    }

    #[test]
    fn test_intersects() {
        let a = Brick::from_str("1,0,1~1,2,1").unwrap();
        let b = Brick::from_str("0,0,2~2,0,2").unwrap();
        assert!(a.intersects(&b));

        let c = Brick::from_str("1,1,8~1,1,9").unwrap();
        assert!(a.intersects(&c));
    }

    #[test]
    fn test_intersects_2() {
        let a = Brick::from_str("6,3,1~6,7,1").unwrap();
        let b = Brick::from_str("6,5,2~6,6,2").unwrap();
        assert!(b.intersects(&a));
    }
    #[test]
    fn test_intersects_against_naive() {
        let snapshot = Snapshot::from_lines(EXAMPLE_INPUT.lines());
        for (a, b) in snapshot.bricks.iter().tuple_combinations() {
            assert_eq!(a.intersects(b), a.intersects_naive(b));
            assert_eq!(b.intersects(a), a.intersects(b))
        }
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(Day22P2::solve(EXAMPLE_INPUT.lines()), "7")
    }
}
