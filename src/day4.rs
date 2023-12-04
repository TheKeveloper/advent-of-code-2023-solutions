use std::collections::HashSet;
use std::str::FromStr;

use crate::common::Solution;

pub enum Day4 {}

impl Solution for Day4 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        lines
            .map(|s| s.as_ref().parse::<Ticket>().unwrap())
            .map(|ticket| ticket.score())
            .sum::<u64>()
            .to_string()
    }
}

#[derive(Default)]
struct Ticket {
    #[allow(dead_code)]
    index: u32,
    own_numbers: HashSet<u8>,
    winning_numbers: HashSet<u8>,
}

impl Ticket {
    pub fn winning_count(&self) -> usize {
        self.own_numbers
            .iter()
            .filter(|&val| self.winning_numbers.contains(val))
            .count()
    }

    pub fn score(&self) -> u64 {
        let winning_count = self.winning_count() as u32;
        if winning_count == 0 {
            0
        } else {
            2u64.pow(winning_count - 1)
        }
    }
}

impl FromStr for Ticket {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(':').collect::<Vec<_>>();
        let [title, ticket_contents] = parts.as_slice() else {
            return Err(anyhow::Error::msg("invalid ticket line").context(s.to_string()));
        };
        let index: u32 = title.trim_start_matches("Card").trim().parse()?;

        let parts = ticket_contents.split('|').collect::<Vec<_>>();
        let [own, winning] = parts.as_slice() else {
            return Err(anyhow::Error::msg("invalid ticket line").context(s.to_string()));
        };

        let own_numbers: HashSet<u8> = own
            .trim()
            .split_ascii_whitespace()
            .map(|val| val.parse().unwrap())
            .collect();
        let winning_numbers: HashSet<u8> = winning
            .trim()
            .split_ascii_whitespace()
            .map(|val| val.parse().unwrap())
            .collect();

        Ok(Ticket {
            index,
            own_numbers,
            winning_numbers,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day4::Day4;

    #[test]
    fn test_example() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        assert_eq!(Day4::solve(input.lines()), "13")
    }
}
