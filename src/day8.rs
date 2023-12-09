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

pub enum Day8P2 {}

impl Solution for Day8P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let network = Network::from_lines(lines);
        network
            .nodes
            .values()
            .filter(|node| node.ends_with_a())
            .map(|node| network.count_steps_to_end_2(node))
            .reduce(lcm)
            .unwrap()
            .to_string()
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
            cur_node = self.get_next(&cur_node, direction)
        }
        steps
    }

    pub fn count_steps_to_end_2(&self, node: &Node) -> usize {
        let mut cur_node = node;
        let mut steps: usize = 0;
        for direction in self.directions.iter().cycle() {
            if cur_node.ends_with_z() {
                return steps;
            }
            steps += 1;
            cur_node = self.get_next(&cur_node, direction)
        }
        steps
    }

    pub fn get_next(&self, node: &Node, direction: &Direction) -> &Node {
        match direction {
            Direction::Left => self.nodes.get(node.left.as_str()).unwrap(),
            Direction::Right => self.nodes.get(node.right.as_str()).unwrap(),
        }
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

    pub fn ends_with_z(&self) -> bool {
        self.name.ends_with('Z')
    }

    pub fn ends_with_a(&self) -> bool {
        self.name.ends_with('A')
    }
}

enum Direction {
    Left,
    Right,
}

fn gcd(a: usize, b: usize) -> usize {
    match (a, b) {
        (0, val) | (val, 0) => val,
        (a, b) => gcd(b, a % b),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
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
    use crate::day8::{Day8, Day8P2};

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

    #[test]
    fn test_example_part2() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
        assert_eq!(Day8P2::solve(input.lines()), "6");
    }
}
