use std::str::FromStr;

use crate::common::Solution;

pub enum Day9 {}

impl Solution for Day9 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        lines
            .map(|line| Sequence::<i64>::from_str(line.as_ref()).unwrap().get_next())
            .sum::<i64>()
            .to_string()
    }
}

pub enum Day9P2 {}

impl Solution for Day9P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        lines
            .map(|line| Sequence::<i64>::from_str(line.as_ref()).unwrap().get_prev())
            .sum::<i64>()
            .to_string()
    }
}

struct Sequence<T> {
    values: Vec<T>,
}

impl FromStr for Sequence<i64> {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(Sequence {
            values: line
                .split_ascii_whitespace()
                .map(|s| {
                    s.parse::<i64>()
                        .map_err(|err| anyhow::Error::msg("Failed to parse int").context(err))
                })
                .collect::<Result<Vec<i64>, anyhow::Error>>()?,
        })
    }
}

impl Sequence<i64> {
    pub fn is_zeros(&self) -> bool {
        self.values.iter().all(|val| *val == 0)
    }

    pub fn get_next(&self) -> i64 {
        if self.is_zeros() {
            return 0;
        }

        let diff_sequence = self.diff_sequence();
        self.values.last().unwrap() + diff_sequence.get_next()
    }

    pub fn get_prev(&self) -> i64 {
        if self.is_zeros() {
            return 0;
        }

        let diff_sequence = self.diff_sequence();
        self.values.first().unwrap() - diff_sequence.get_prev()
    }

    fn diff_sequence(&self) -> Sequence<i64> {
        self.values
            .windows(2)
            .map(|parts| {
                let [first, second] = parts else {
                    panic!("Did not get window of size 2")
                };
                second - first
            })
            .collect::<Vec<_>>()
            .into()
    }
}

impl<T> From<Vec<T>> for Sequence<T> {
    fn from(values: Vec<T>) -> Self {
        Sequence { values }
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day9::{Day9, Day9P2};

    const EXAMPLE_INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
    #[test]
    fn test_example() {
        assert_eq!(Day9::solve(EXAMPLE_INPUT.lines()), "114");
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(Day9P2::solve(EXAMPLE_INPUT.lines()), "2");
    }
}
