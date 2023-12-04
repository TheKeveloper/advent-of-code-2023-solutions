use advent_of_code_2023_solutions::{Solution};
use std::str::FromStr;

fn main() {
    Day2::default_print_solution();
}
struct Day2 {}
impl Solution for Day2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> u32 {
        let mut sum: u32 = 0;
        for line in lines {
            let line = line.as_ref();
            if line.is_empty() {
                continue;
            }
            let game: Game = line.parse().expect("Could not parse game");
            if game.is_feasible() {
                sum += game.index;
            }
        }
        sum
    }
}

#[derive(Debug)]
struct Game {
    index: u32,
    sets: Vec<Set>,
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(": ").collect::<Vec<_>>();
        let [game_prefix, set_definitions] = parts.as_slice() else {
            println!("Parts: {:?}", parts);
            return Err(anyhow::Error::msg("Invalid game received").context(s.to_string()));
        };

        let index: u32 = game_prefix.trim_start_matches("Game ").parse()?;
        let mut sets: Vec<Set> = Vec::new();

        for set_definition in set_definitions.split("; ") {
            let set: Set = set_definition.parse()?;
            sets.push(set);
        }

        Ok(Game { index, sets })
    }
}

impl Game {
    pub fn is_feasible(&self) -> bool {
        self.sets.iter().all(Set::is_feasible)
    }
}

impl FromStr for Set {
    type Err = anyhow::Error;

    /// Expected format is "<number> red, <number> green"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set: Set = Set::default();
        for part in s.trim().split(", ") {
            let (color, count) = parse_color_and_count(part)?;
            match color {
                CubeColor::Red => set.red = count,
                CubeColor::Green => set.green = count,
                CubeColor::Blue => set.blue = count,
            };
        }
        Ok(set)
    }
}

/// Expected format is "<number> red"
fn parse_color_and_count(s: &str) -> Result<(CubeColor, u32), anyhow::Error> {
    let parts: Vec<_> = s.trim().split_ascii_whitespace().collect();

    match parts.as_slice() {
        [count, color] => {
            let count: u32 = count.parse()?;
            let color: CubeColor = color.parse()?;
            Ok((color, count))
        }
        _ => Err(anyhow::Error::msg("Invalid color and count").context(s.to_string())),
    }
}

impl Set {
    pub fn is_feasible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

impl FromStr for CubeColor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(CubeColor::Red),
            "green" => Ok(CubeColor::Green),
            "blue" => Ok(CubeColor::Blue),
            _ => Err(anyhow::Error::msg("Invalid string received").context(s.to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_example() {
        let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!(Day2::solve(input.lines()), 8);
    }

    #[test]
    fn test_parse_count_and_color() {
        assert_eq!(
            parse_color_and_count("3 blue").unwrap(),
            (CubeColor::Blue, 3)
        );
    }

    #[test]
    fn test_parse_set() {
        assert_eq!(
            Set::from_str("8 green, 6 blue, 20 red").unwrap(),
            Set {
                green: 8,
                blue: 6,
                red: 20
            }
        );
    }
}
