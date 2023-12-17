use crate::common::Solution;

pub enum Day21 {}

impl Solution for Day21 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        panic!(
            "lines: {:?}",
            lines.map(|s| s.as_ref().to_string()).collect::<Vec<_>>()
        )
    }
}

pub enum Day21P2 {}
impl Solution for Day21P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        panic!(
            "lines: {:?}",
            lines.map(|s| s.as_ref().to_string()).collect::<Vec<_>>()
        )
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day21::{Day21, Day21P2};

    const EXAMPLE_INPUT: &str = r"";
    #[test]
    #[should_panic]
    fn test_example() {
        assert_eq!(Day21::solve(EXAMPLE_INPUT.lines()), "")
    }

    #[test]
    #[should_panic]
    fn test_example_p2() {
        assert_eq!(Day21P2::solve(EXAMPLE_INPUT.lines()), "")
    }
}
