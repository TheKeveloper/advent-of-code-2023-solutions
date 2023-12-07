use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;
use std::str::FromStr;

use crate::common::Solution;

pub enum Day7 {}

impl Solution for Day7 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let mut hands: Vec<Hand> = lines.map(|line| line.as_ref().parse().unwrap()).collect();
        let count = hands.len();
        hands.sort();

        hands
            .iter()
            .enumerate()
            .map(|(index, hand)| ((count - index) as u64) * hand.bid)
            .sum::<u64>()
            .to_string()
    }
}

static CARD_ORDERING: &'static [char] = &[
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

struct Hand {
    original: String,
    ordered_card_counts: Vec<(char, usize)>,
    bid: u64,
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        let [cards, bid] = parts.as_slice() else {
            return Err(anyhow::Error::msg("Invalid input line").context(line.to_string()));
        };

        Ok(Hand {
            original: cards.to_string(),
            ordered_card_counts: get_ordered_count(cards),
            bid: bid.parse()?,
        })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.ordered_card_counts.eq(&other.ordered_card_counts)
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for ((_, self_count), (_, other_count)) in
            zip(&self.ordered_card_counts, &other.ordered_card_counts)
        {
            // N.B. need to get the lesser card so reverse order of compare
            match other_count.cmp(self_count) {
                Ordering::Equal => {}
                unequal => return Some(unequal),
            }
        }

        for (self_card, other_card) in &mut self.original.chars().zip(&mut other.original.chars()) {
            match compare_card(self_card, other_card) {
                Ordering::Equal => {}
                unequal => return Some(unequal),
            }
        }
        Some(Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn get_ordered_count(cards: &str) -> Vec<(char, usize)> {
    let count_map = cards.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let mut counts: Vec<(char, usize)> = count_map.into_iter().collect();

    counts.sort_by(compare_card_and_count);

    counts
}

fn compare_card_and_count(
    (a_card, a_count): &(char, usize),
    (b_card, b_count): &(char, usize),
) -> Ordering {
    // N.B.: represent "stronger" hands as "less", so need to sort in descending order by count
    match b_count.cmp(a_count) {
        Ordering::Equal => compare_card(*a_card, *b_card),
        other => other,
    }
}

fn compare_card(a: char, b: char) -> Ordering {
    let a_index = CARD_ORDERING.iter().position(|c| *c == a).unwrap();
    let b_index = CARD_ORDERING.iter().position(|c| *c == b).unwrap();

    a_index.cmp(&b_index)
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day7::Day7;

    const INPUT: &'static str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    #[test]
    fn test_example() {
        assert_eq!(Day7::solve(INPUT.lines()), "6440")
    }
}
