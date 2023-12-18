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

    const EXAMPLE_INPUT: &str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    #[test]
    fn test_example() {
        assert_eq!(Day18::solve(EXAMPLE_INPUT.lines()), "62")
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(Day18P2::solve(EXAMPLE_INPUT.lines()), "")
    }
}
