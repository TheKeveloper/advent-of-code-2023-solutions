use crate::common::Solution;

pub enum Day15 {}
impl Solution for Day15 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        todo!()
    }
}

pub enum Day15P2 {}
impl Solution for Day15P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day15::{Day15, Day15P2};

    const EXAMPLE_INPUT: &'static str = r"";

    #[test]
    fn test_example() {
        assert_eq!(Day15::solve(EXAMPLE_INPUT.lines()), "")
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(Day15P2::solve(EXAMPLE_INPUT.lines()), "")
    }
}
