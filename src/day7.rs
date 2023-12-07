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

pub enum Day7P2 {}

impl Solution for Day7P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let mut hands: Vec<Hand> = lines.map(|line| line.as_ref().parse().unwrap()).collect();
        let count = hands.len();
        hands.sort_by(compare_hands_2);

        hands
            .iter()
            .enumerate()
            .map(|(index, hand)| ((count - index) as u64) * hand.bid)
            .sum::<u64>()
            .to_string()
    }
}

static CARD_ORDERING: &[char] = &[
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

static CARD_ORDERING_2: &[char] = &[
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
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
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        for ((_, self_count), (_, other_count)) in
            zip(&self.ordered_card_counts, &other.ordered_card_counts)
        {
            // N.B. need to get the lesser card so reverse order of compare
            match other_count.cmp(self_count) {
                Ordering::Equal => {}
                unequal => return unequal,
            }
        }

        for (self_card, other_card) in &mut self.original.chars().zip(&mut other.original.chars()) {
            match compare_card(self_card, other_card) {
                Ordering::Equal => {}
                unequal => return unequal,
            }
        }
        Ordering::Equal
    }
}

fn compare_hands_2(first: &Hand, second: &Hand) -> Ordering {
    let first_js_count = first
        .ordered_card_counts
        .iter()
        .find_map(|(char, count)| if *char == 'J' { Some(*count) } else { None })
        .unwrap_or(0usize);
    let second_js_count = second
        .ordered_card_counts
        .iter()
        .find_map(|(char, count)| if *char == 'J' { Some(*count) } else { None })
        .unwrap_or(0usize);

    let mut first_ordered: Vec<_> = first
        .ordered_card_counts
        .iter()
        .copied()
        .filter(|(card, _)| *card != 'J')
        .collect();

    if let Some((_, count)) = first_ordered.get_mut(0) {
        *count += first_js_count
    }
    if first_ordered.is_empty() {
        first_ordered.push(('J', 5usize));
    }

    let mut second_ordered: Vec<_> = second
        .ordered_card_counts
        .iter()
        .copied()
        .filter(|(card, _)| *card != 'J')
        .collect();

    if let Some((_, count)) = second_ordered.get_mut(0) {
        *count += second_js_count
    }
    if second_ordered.is_empty() {
        second_ordered.push(('J', 5usize));
    }

    for ((_, first_count), (_, second_count)) in zip(&first_ordered, &second_ordered) {
        // N.B. need to get the lesser card so reverse order of compare
        match second_count.cmp(first_count) {
            Ordering::Equal => {}
            unequal => return unequal,
        }
    }

    for (self_card, other_card) in &mut first.original.chars().zip(&mut second.original.chars()) {
        match compare_card_2(self_card, other_card) {
            Ordering::Equal => {}
            unequal => return unequal,
        }
    }
    Ordering::Equal
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

fn compare_card_2(a: char, b: char) -> Ordering {
    let a_index = CARD_ORDERING_2.iter().position(|c| *c == a).unwrap();
    let b_index = CARD_ORDERING_2.iter().position(|c| *c == b).unwrap();

    a_index.cmp(&b_index)
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day7::{Day7, Day7P2};

    const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    #[test]
    fn test_example() {
        assert_eq!(Day7::solve(INPUT.lines()), "6440")
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(Day7P2::solve(INPUT.lines()), "5905")
    }
}
