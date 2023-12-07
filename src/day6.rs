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
            .product::<u64>()
            .to_string()
    }
}

pub enum Day6P2 {}

impl Solution for Day6P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let lines: Vec<_> = lines.collect();
        let time = lines[0]
            .as_ref()
            .strip_prefix("Time:")
            .unwrap()
            .replace(' ', "")
            .parse::<u64>()
            .unwrap();

        let distance = lines[1]
            .as_ref()
            .strip_prefix("Distance:")
            .unwrap()
            .replace(' ', "")
            .parse::<u64>()
            .unwrap();

        let race = Race { time, distance };
        race.get_record_setting_ways().to_string()
    }
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn get_record_setting_ways(&self) -> u64 {
        // we win if (time - acc) * acc > distance
        // this yields a quadratic equation that we can use
        // - acc^2 + time * acc - distance > 0
        let (first, second) = solve_quadratic(
            -1f64,
            1f64 * self.time as f64,
            -1f64 * (self.distance as f64),
        );

        // there's probably a more principled way to do this, but this basically just gets rid of
        // any weird rounding errors that we may have to make sure the ones we chosen are winnable
        let (smaller, larger) = (f64::min(first, second), f64::max(first, second));
        let (mut smaller, mut larger) = (f64::floor(smaller) as u64, f64::ceil(larger) as u64);
        while !self.can_win(smaller) {
            smaller += 1
        }
        while !self.can_win(larger) {
            larger -= 1
        }

        larger - smaller + 1
    }

    fn can_win(&self, acc_time: u64) -> bool {
        if acc_time > self.time {
            return false;
        }
        acc_time * (self.time - acc_time) > self.distance
    }
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    (
        (-b + f64::sqrt(b.powi(2) - (4f64 * a * c))) / (2f64 * a),
        (-b - f64::sqrt(b.powi(2) - (4f64 * a * c))) / (2f64 * a),
    )
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day6::{Day6, Day6P2};

    const INPUT: &'static str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_example() {
        assert_eq!(Day6::solve(INPUT.lines()), "288");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day6P2::solve(INPUT.lines()), "71503");
    }
}
