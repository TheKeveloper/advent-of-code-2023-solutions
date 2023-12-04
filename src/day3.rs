use crate::common::Solution;
use crate::vec2d::{CellRowRange, Vec2d};

pub enum Day3 {}

impl Solution for Day3 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let matrix: Vec2d<char> = Vec2d::from_lines(lines);
        get_values(&matrix).sum::<u32>().to_string()
    }
}

fn get_values(matrix: &Vec2d<char>) -> impl Iterator<Item = u32> + '_ {
    get_numeric_ranges(matrix)
        .into_iter()
        .filter(|range| borders_symbol(range))
        .map(|range| range.to_string())
        .map(|s| s.parse::<u32>().unwrap())
}

fn is_symbol(c: &char) -> bool {
    !c.is_numeric() && *c != '.'
}

fn borders_symbol(range: &CellRowRange<char>) -> bool {
    for cell in range.cells() {
        for neighbor in cell.neighbors() {
            if is_symbol(neighbor.value()) {
                return true;
            }
        }
    }
    false
}

fn get_numeric_ranges(matrix: &Vec2d<char>) -> Vec<CellRowRange<'_, char>> {
    let mut ranges = Vec::new();

    for (row, row_vec) in matrix.inner.iter().enumerate() {
        let mut start: Option<usize> = None;
        for (col, val) in row_vec.iter().enumerate() {
            if val.is_ascii_digit() {
                match start {
                    None => start = Some(col),
                    Some(_) => {}
                }
            } else {
                match start {
                    None => {}
                    Some(first_col) => {
                        ranges.push(matrix.get_range(row, first_col, col - 1));
                        start = None
                    }
                }
            }
        }
        match start {
            None => {}
            Some(first_col) => ranges.push(matrix.get_range(row, first_col, row_vec.len() - 1)),
        }
    }
    ranges
}

#[cfg(test)]
mod test {
    use crate::day3::*;

    #[test]
    fn test_example() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(Day3::solve(input.lines()), 4361)
    }

    #[test]
    fn test_get_values() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let matrix: Vec2d<char> = Vec2d::from_lines(input.lines());
        let values: Vec<u32> = get_values(&matrix).collect();
        assert_eq!(values, vec![467, 35, 633, 617, 592, 755, 664, 598]);
    }
}
