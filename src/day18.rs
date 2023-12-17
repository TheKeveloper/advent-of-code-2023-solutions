use crate::common::Solution;

pub enum Day18 {}

impl Solution for Day18 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        panic!(
            "lines: {:?}",
            lines.map(|s| s.as_ref().to_string()).collect::<Vec<_>>()
        )
    }
}

pub enum Day18P2 {}
impl Solution for Day18P2 {
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
    use crate::day18::{Day18, Day18P2};

    const EXAMPLE_INPUT: &'static str = r"";
    #[test]
    #[should_panic]
    fn test_example() {
        assert_eq!(Day18::solve(EXAMPLE_INPUT.lines()), "")
    }

    #[test]
    #[should_panic]
    fn test_example_p2() {
        assert_eq!(Day18P2::solve(EXAMPLE_INPUT.lines()), "")
    }
}
