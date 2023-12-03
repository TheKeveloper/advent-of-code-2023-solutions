use advent_of_code_2023_solutions::{get_input_lines, Solution};

fn main() {
    println!("{}", Day1::solve(get_input_lines()));
}
struct Day1 {}

impl Solution for Day1 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> u32 {
        lines.map(|line| get_number(line.as_ref())).sum()
    }
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

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn test_example() {
        let input = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

        assert_eq!(solve(input.lines()), 142);
    }
}
