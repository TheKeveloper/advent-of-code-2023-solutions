use crate::common::Solution;

pub enum Day25 {}

impl Solution for Day25 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        panic!(
            "lines: {:?}",
            lines.map(|s| s.as_ref().to_string()).collect::<Vec<_>>()
        )
    }
}

pub enum Day25P2 {}
impl Solution for Day25P2 {
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
    use crate::day25::{Day25, Day25P2};

    const EXAMPLE_INPUT: &str = r"";
    #[test]
    #[should_panic]
    fn test_example() {
        assert_eq!(Day25::solve(EXAMPLE_INPUT.lines()), "")
    }

    #[test]
    #[should_panic]
    fn test_example_p2() {
        assert_eq!(Day25P2::solve(EXAMPLE_INPUT.lines()), "")
    }
}
