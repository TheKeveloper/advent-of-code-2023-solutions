use std::iter::zip;

use crate::common::Solution;

pub enum Day6 {}

impl Solution for Day6 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let lines: Vec<_> = lines.collect();
        let times = lines[0]
            .as_ref()
            .strip_prefix("Time:")
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap());

        let distances = lines[1]
            .as_ref()
            .strip_prefix("Distance:")
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap());

        zip(times, distances)
            .map(|(time, distance)| Race { time, distance })
            .map(|race| race.get_record_setting_ways())
            .product::<usize>()
            .to_string()
    }
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn get_record_setting_ways(&self) -> usize {
        (1..self.time)
            .filter(|acc| acc * (self.time - acc) > self.distance)
            .count()
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day6::Day6;

    const INPUT: &'static str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_example() {
        assert_eq!(Day6::solve(INPUT.lines()), "288");
    }
}
