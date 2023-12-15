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
                    .sum::<u64>()
            })
            .sum::<u64>()
            .to_string()
    }
}

fn update_hash(cur: u64, c: char) -> u64 {
    ((cur + (c as u64)) * 17) % 256
}

pub enum Day15P2 {}
impl Solution for Day15P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day15::{Day15, Day15P2};

    const EXAMPLE_INPUT: &'static str = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_example() {
        assert_eq!(Day15::solve(EXAMPLE_INPUT.lines()), "1320")
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(Day15P2::solve(EXAMPLE_INPUT.lines()), "")
    }
}
