use crate::common::Solution;

pub enum Day19 {}

impl Solution for Day19 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        panic!(
            "lines: {:?}",
            lines.map(|s| s.as_ref().to_string()).collect::<Vec<_>>()
        )
    }
}

pub enum Day19P2 {}
impl Solution for Day19P2 {
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
    use crate::day19::{Day19, Day19P2};

    const EXAMPLE_INPUT: &str = r"";
    #[test]
    #[should_panic]
    fn test_example() {
        assert_eq!(Day19::solve(EXAMPLE_INPUT.lines()), "")
    }

    #[test]
    #[should_panic]
    fn test_example_p2() {
        assert_eq!(Day19P2::solve(EXAMPLE_INPUT.lines()), "")
    }
}
