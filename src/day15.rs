use std::str::FromStr;

use itertools::Itertools;

use crate::common::Solution;

pub enum Day15 {}
impl Solution for Day15 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        lines
            .map(|line| {
                line.as_ref()
                    .split(',')
                    .map(|s| s.chars().fold(0, update_hash))
                    .sum::<usize>()
            })
            .sum::<usize>()
            .to_string()
    }
}

pub enum Day15P2 {}
impl Solution for Day15P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let mut map = LensHashMap::new();
        lines
            .flat_map(|line| {
                line.as_ref()
                    .split(',')
                    .map(|s| s.parse::<LensOp>().unwrap())
                    .collect_vec()
            })
            .for_each(|val| map.perform_op(val));

        map.get_summary().to_string()
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Lens {
    label: String,
    focal_length: usize,
}

enum LensOp {
    Put(Lens),
    Remove(String),
}

pub fn hash_label(label: &str) -> usize {
    label.chars().fold(0, update_hash)
}

struct LensHashMap {
    boxes: Vec<Vec<Lens>>,
}

impl LensHashMap {
    pub fn new() -> LensHashMap {
        LensHashMap {
            boxes: vec![Vec::new(); 256],
        }
    }

    pub fn insert(&mut self, lens: Lens) {
        let bucket = self.boxes.get_mut(hash_label(lens.label.as_str())).unwrap();
        let existing = bucket.iter_mut().find(|other| other.label.eq(&lens.label));
        match existing {
            None => bucket.push(lens),
            Some(existing) => existing.focal_length = lens.focal_length,
        }
    }

    pub fn remove(&mut self, label: &str) {
        let bucket = self.boxes.get_mut(hash_label(label)).unwrap();
        let index = bucket.iter().find_position(|other| other.label.eq(label));
        if let Some((index, _)) = index {
            bucket.remove(index);
        }
    }

    pub fn perform_op(&mut self, op: LensOp) {
        match op {
            LensOp::Put(lens) => self.insert(lens),
            LensOp::Remove(label) => self.remove(label.as_str()),
        }
    }

    pub fn get_summary(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .flat_map(|(box_n, box_val)| {
                box_val
                    .iter()
                    .enumerate()
                    .map(move |(index, lens)| (box_n + 1) * (index + 1) * lens.focal_length)
            })
            .sum()
    }
}

fn update_hash(cur: usize, c: char) -> usize {
    ((cur + (c as usize)) * 17) % 256
}

impl FromStr for LensOp {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (index, op) = s
            .chars()
            .enumerate()
            .find(|(_, c)| *c == '=' || *c == '-')
            .unwrap();
        match op {
            '=' => {
                let (label, focal_length) = s.split_once('=').unwrap();
                Ok(LensOp::Put(Lens {
                    label: label.to_string(),
                    focal_length: focal_length.parse()?,
                }))
            }
            '-' => {
                let (label, _) = s.split_at(index);
                Ok(LensOp::Remove(label.to_string()))
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day15::{Day15, Day15P2};

    const EXAMPLE_INPUT: &str = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_example() {
        assert_eq!(Day15::solve(EXAMPLE_INPUT.lines()), "1320")
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(Day15P2::solve(EXAMPLE_INPUT.lines()), "145")
    }
}
