use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use priority_queue::PriorityQueue;

use crate::common::Solution;
use crate::vec2d::{Direction, RowCol, Vec2d};

pub enum Day17 {}

impl Solution for Day17 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let grid = Grid::from_lines(lines);
        grid.get_shortest_path().to_string()
    }
}

pub enum Day17P2 {}
impl Solution for Day17P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let grid = Grid::from_lines(lines);
        grid.get_shortest_path_2().to_string()
    }
}

struct Grid {
    blocks: Vec2d<u8>,
}

impl Grid {
    pub fn from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Grid {
        Grid {
            blocks: Vec2d::from_lines(lines).map(|c| {
                c.to_digit(10)
                    .unwrap_or_else(|| panic!("Expected a digit value: {}", c))
                    as u8
            }),
        }
    }

    pub fn get_shortest_path(&self) -> usize {
        // just using Djikstra's algorithm for this
        let mut queue: PriorityQueue<DirectionalNode, Reverse<usize>> = PriorityQueue::new();
        let mut distances: HashMap<DirectionalNode, usize> = HashMap::new();

        let starting_node = self.starting_node();
        queue.push(starting_node, Reverse(0));
        distances.insert(starting_node, 0);

        while let Some((node, Reverse(node_dist))) = queue.pop() {
            for neighbor in self.neighbors(&node) {
                let new_dist = node_dist + self.value(&neighbor);
                match distances.entry(neighbor) {
                    Entry::Occupied(mut occupied) => {
                        if new_dist < *occupied.get() {
                            occupied.insert(new_dist);
                            queue.push_increase(neighbor, Reverse(new_dist));
                        }
                    }
                    Entry::Vacant(vacant) => {
                        vacant.insert(new_dist);
                        queue.push_increase(neighbor, Reverse(new_dist));
                    }
                }
            }
        }

        let (_, dist) = distances
            .into_iter()
            .filter(|(node, _)| self.is_end(node))
            .min_by_key(|(_, val)| *val)
            .unwrap();
        dist
    }

    pub fn get_shortest_path_2(&self) -> usize {
        // just using Djikstra's algorithm for this
        let mut queue: PriorityQueue<DirectionalNode, Reverse<usize>> = PriorityQueue::new();
        let mut distances: HashMap<DirectionalNode, usize> = HashMap::new();

        let starting_node = self.starting_node();
        queue.push(starting_node, Reverse(0));
        distances.insert(starting_node, 0);

        while let Some((node, Reverse(node_dist))) = queue.pop() {
            for neighbor in self.neighbors_2(&node) {
                let new_dist = node_dist + self.value_2(&neighbor);
                match distances.entry(neighbor) {
                    Entry::Occupied(mut occupied) => {
                        if new_dist < *occupied.get() {
                            occupied.insert(new_dist);
                            queue.push_increase(neighbor, Reverse(new_dist));
                        }
                    }
                    Entry::Vacant(vacant) => {
                        vacant.insert(new_dist);
                        queue.push_increase(neighbor, Reverse(new_dist));
                    }
                }
            }
        }

        let (_, dist) = distances
            .into_iter()
            .filter(|(node, _)| self.is_end(node) && node.direction_count >= 4)
            .min_by_key(|(_, val)| *val)
            .unwrap();
        dist
    }

    pub fn value(&self, node: &DirectionalNode) -> usize {
        *self.blocks.get(node.coords.row, node.coords.col).unwrap() as usize
    }
    pub fn value_2(&self, node: &DirectionalNode) -> usize {
        if self.is_end(node) && node.direction_count < 4 {
            usize::MAX / 2
        } else {
            *self.blocks.get(node.coords.row, node.coords.col).unwrap() as usize
        }
    }

    fn is_end(&self, node: &DirectionalNode) -> bool {
        node.coords.eq(&self.get_end())
    }

    pub fn get_end(&self) -> RowCol {
        RowCol {
            row: self.blocks.num_rows() - 1,
            col: self.blocks.first_num_cols() - 1,
        }
    }

    pub fn starting_node(&self) -> DirectionalNode {
        DirectionalNode {
            coords: RowCol { row: 0, col: 0 },
            // arbitrarily pick the direction as up since we can't actually go up
            direction: Direction::Right,
            direction_count: 0,
        }
    }

    pub fn neighbors<'a>(
        &self,
        node: &'a DirectionalNode,
    ) -> impl Iterator<Item = DirectionalNode> + 'a {
        let num_rows = self.blocks.num_rows();
        let num_cols = self.blocks.first_num_cols();
        [
            Direction::Up,
            Direction::Left,
            Direction::Right,
            Direction::Down,
        ]
        .iter()
        .filter(|&direction| {
            (direction.ne(&node.direction) || node.direction_count < 3)
                // avoid reversing direction
                && direction.opposite().ne(&node.direction)
        })
        .filter_map(|direction| {
            direction.next(node.coords).map(|coords| DirectionalNode {
                direction: *direction,
                coords,
                direction_count: if direction.eq(&node.direction) {
                    node.direction_count + 1
                } else {
                    1
                },
            })
        })
        .filter(move |next| next.coords.row < num_rows && next.coords.col < num_cols)
    }

    pub fn neighbors_2<'a>(
        &self,
        node: &'a DirectionalNode,
    ) -> impl Iterator<Item = DirectionalNode> + 'a {
        let num_rows = self.blocks.num_rows();
        let num_cols = self.blocks.first_num_cols();
        [
            Direction::Up,
            Direction::Left,
            Direction::Right,
            Direction::Down,
        ]
        .iter()
        .filter(|&direction| {
            // avoid reversing direction
            node.coords.row == 0 && node.coords.col == 0
                || (direction.opposite().ne(&node.direction)
                    && (direction.ne(&node.direction) || node.direction_count < 10)
                    && (direction.eq(&node.direction) || node.direction_count >= 4))
        })
        .filter_map(|direction| {
            direction.next(node.coords).map(|coords| DirectionalNode {
                direction: *direction,
                coords,
                direction_count: if direction.eq(&node.direction) {
                    node.direction_count + 1
                } else {
                    1
                },
            })
        })
        .filter(move |next| next.coords.row < num_rows && next.coords.col < num_cols)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
struct DirectionalNode {
    coords: RowCol,
    direction: Direction,
    direction_count: usize,
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day17::{Day17, Day17P2};

    const EXAMPLE_INPUT: &str = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    #[test]
    fn test_example() {
        assert_eq!(Day17::solve(EXAMPLE_INPUT.lines()), "102")
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(Day17P2::solve(EXAMPLE_INPUT.lines()), "94")
    }

    #[test]
    fn test_example_2_p2() {
        let input = r#"111111111111
999999999991
999999999991
999999999991
999999999991"#;
        assert_eq!(Day17P2::solve(input.lines()), "71")
    }
}
