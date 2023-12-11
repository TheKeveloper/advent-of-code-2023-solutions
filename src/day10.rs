use crate::common::Solution;

pub enum Day10 {}

impl Solution for Day10 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day10::Day10;

    const SIMPLE_EXAMPLE_INPUT: &'static str = r#".....
.S-7.
.|.|.
.L-J.
....."#;

    const COMPLEX_EXAMPLE_INPUT: &'static str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

    #[test]
    fn test_examples() {
        assert_eq!(Day10::solve(SIMPLE_EXAMPLE_INPUT.lines()), "4");
        assert_eq!(Day10::solve(COMPLEX_EXAMPLE_INPUT.lines()), "8");
    }
}
