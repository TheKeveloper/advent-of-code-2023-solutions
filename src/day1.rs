use std::str::FromStr;

use regex::Regex;

use crate::common::Solution;

pub enum Day1 {}

pub enum Day1P2 {}

impl Solution for Day1 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        lines
            .map(|line| get_number(line.as_ref()))
            .sum::<u32>()
            .to_string()
    }
}

impl Solution for Day1P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let match_values = [
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "one", "two", "three", "four",
            "five", "six", "seven", "eight", "nine", "zero",
        ];

        let forward_pattern = match_values.join("|");
        let backward_pattern = match_values
            .into_iter()
            .map(reverse_string)
            .collect::<Vec<String>>()
            .join("|");
        let forward_regex = Regex::from_str(forward_pattern.as_str()).unwrap();

        let backward_regex = Regex::from_str(backward_pattern.as_str()).unwrap();
        lines
            .map(|line| get_number_part2(&forward_regex, &backward_regex, line.as_ref()))
            .sum::<u32>()
            .to_string()
    }
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn get_number(line: &str) -> u32 {
    if line.is_empty() {
        return 0;
    }
    let first_digit = line
        .chars()
        .find_map(|c| c.to_digit(10))
        .expect("Did not find first digit");
    let last_digit = line
        .chars()
        .rev()
        .find_map(|c| c.to_digit(10))
        .expect("Did not find last digit");
    10 * first_digit + last_digit
}

fn get_number_part2(forward_regex: &Regex, backward_regex: &Regex, line: &str) -> u32 {
    let first = forward_regex
        .find(line)
        .expect("No substring match found")
        .as_str();
    let last = reverse_string(
        backward_regex
            .find(reverse_string(line).as_str())
            .expect("No match for last_regex found")
            .as_str(),
    );

    let first_digit = to_digit(first);
    let last_digit = to_digit(last.as_str());
    first_digit * 10 + last_digit
}

fn to_digit(s: &str) -> u32 {
    match s {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("Attempted to convert invalid string to digit: {}", s),
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day1::{Day1, Day1P2};

    #[test]
    fn test_day1_example() {
        const INPUT: &str = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;
        assert_eq!(Day1::solve(INPUT.lines()), "142");
    }

    #[test]
    fn test_day1p2_example() {
        const INPUT: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;
        assert_eq!(Day1P2::solve(INPUT.lines()), "281")
    }

    #[test]
    fn test_tricky_day1p2() {
        let input = "oneight";
        assert_eq!(Day1P2::solve(input.lines()), "18");
    }
}
