use std::collections::HashMap;
use std::str::FromStr;

use crate::common::Solution;

pub enum Day8 {}

impl Solution for Day8 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let network = Network::from_lines(lines);
        network.count_steps_to_end().to_string()
    }
}

struct Network {
    directions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

impl Network {
    pub fn from_lines(mut lines: impl Iterator<Item = impl AsRef<str>>) -> Network {
        let mut nodes: HashMap<String, Node> = HashMap::new();
        let first_line = lines.next().unwrap();
        let first_line = first_line.as_ref();
        let directions: Vec<Direction> =
            first_line.chars().map(|c| c.try_into().unwrap()).collect();
        let _ = lines.next();

        for line in lines {
            let node: Node = line.as_ref().parse().unwrap();
            nodes.insert(node.name.clone(), node);
        }

        Network { directions, nodes }
    }

    pub fn count_steps_to_end(&self) -> usize {
        let mut cur_node = self.nodes.get("AAA").unwrap();
        let mut steps: usize = 0;
        for direction in self.directions.iter().cycle() {
            if cur_node.is_end() {
                return steps;
            }
            steps += 1;
            match direction {
                Direction::Left => {
                    cur_node = self.nodes.get(cur_node.left.as_str()).unwrap();
                }
                Direction::Right => cur_node = self.nodes.get(cur_node.right.as_str()).unwrap(),
            }
        }
        steps
    }
}

struct Node {
    name: String,
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (name, directions) = line.split_once(" = ").unwrap();
        let (left, right) = directions
            .trim()
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();
        Ok(Node {
            name: name.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        })
    }
}

impl Node {
    pub fn is_end(&self) -> bool {
        self.name.eq("ZZZ")
    }
}

enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Direction::Right),
            'L' => Ok(Direction::Left),
            _ => Err(anyhow::Error::msg("Invalid character received").context(value)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day8::Day8;

    const INPUT1: &'static str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    const INPUT2: &'static str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
    #[test]
    fn test_example_cases() {
        assert_eq!(Day8::solve(INPUT1.lines()), "2");
        assert_eq!(Day8::solve(INPUT2.lines()), "6");
    }
}
