use std::collections::HashMap;
use std::rc::Rc;

use itertools::Itertools;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

use crate::common::Solution;

pub enum Day25 {}

impl Solution for Day25 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let graph = Graph::from_lines(lines);
        let mut rng = StdRng::seed_from_u64(1);
        loop {
            let result = graph.karger(&mut rng);
            if result.remaining_edges.len() == 3 {
                return (result.partitions.0.len() * result.partitions.1.len()).to_string();
            }
        }
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

struct Graph {
    edges: Vec<Edge>,
}

impl Graph {
    pub fn from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Graph {
        Graph {
            edges: lines.flat_map(|s| Edge::parse_line(s.as_ref())).collect(),
        }
    }

    pub fn karger<R>(&self, rng: &mut R) -> KargerResult
    where
        R: Rng + Sized,
    {
        let shuffled_edges = {
            let mut edges = self.edges.clone();
            edges.shuffle(rng);
            edges
        };
        let mut supernodes: SuperNodeCollection = SuperNodeCollection {
            nodes: self
                .get_nodes()
                .into_iter()
                .map(|name| {
                    (
                        name.clone(),
                        SuperNode {
                            parent: Rc::new(name),
                            rank: 0,
                        },
                    )
                })
                .collect(),
        };

        let mut num_nodes = supernodes.nodes.len();

        for edge in &shuffled_edges {
            let partitions: Vec<Vec<String>> = self
                .get_nodes()
                .into_iter()
                .group_by(|node| supernodes.find_root(node.as_str()).as_str().to_string())
                .into_iter()
                .map(|(_, group)| group.collect())
                .collect();
            if num_nodes == 2 && partitions.len() <= 2 {
                break;
            }
            let first = supernodes.find_root(edge.0.as_str());
            let second = supernodes.find_root(edge.1.as_str());
            if first.as_str().eq_ignore_ascii_case(second.as_str()) {
                println!("Found same edge");
                continue;
            }

            supernodes.union(first.as_str(), second.as_str());
            let after_partitions: Vec<Vec<String>> = self
                .get_nodes()
                .into_iter()
                .group_by(|node| supernodes.find_root(node.as_str()).as_str().to_string())
                .into_iter()
                .map(|(_, group)| group.collect())
                .collect();

            if after_partitions.len() != partitions.len() - 1 {
                panic!("Got part")
            }
            num_nodes -= 1;
        }
        debug_assert_eq!(num_nodes, 2);

        let remaining_edges: Vec<Edge> = self
            .edges
            .iter()
            .filter(|edge| {
                let first = supernodes.find_root(edge.0.as_str());
                let second = supernodes.find_root(edge.1.as_str());
                first.as_str().ne(second.as_str())
            })
            .cloned()
            .collect();

        let partitions: Vec<Vec<String>> = self
            .get_nodes()
            .into_iter()
            .group_by(|node| supernodes.find_root(node.as_str()).as_str().to_string())
            .into_iter()
            .map(|(_, group)| group.collect())
            .collect();

        let [first, second] = partitions.as_slice() else {
            panic!("Did not receive exactly two groups: {:?}", partitions);
        };

        KargerResult {
            partitions: (first.clone(), second.clone()),
            remaining_edges,
        }
    }

    fn get_nodes(&self) -> Vec<String> {
        self.edges
            .iter()
            .flat_map(|Edge(a, b)| [a.clone(), b.clone()].into_iter())
            .unique()
            .collect()
    }
}

struct KargerResult {
    partitions: (Vec<String>, Vec<String>),
    remaining_edges: Vec<Edge>,
}

#[derive(Debug)]
struct SuperNodeCollection {
    nodes: HashMap<String, SuperNode>,
}

impl SuperNodeCollection {
    pub fn find_root(&mut self, name: &str) -> Rc<String> {
        let node_parent = self.nodes.get(name).unwrap().parent.clone();
        if node_parent.as_str().eq_ignore_ascii_case(name) {
            return node_parent;
        }
        let root = self.find_root(node_parent.as_str());
        let node = self.nodes.get_mut(name).unwrap();
        node.parent = root.clone();
        root
    }

    pub fn union(&mut self, first: &str, second: &str) {
        let first = self.find_root(first);
        let second = self.find_root(second);

        let first_rank = self.nodes.get(first.as_ref()).unwrap().rank;
        let second_rank = self.nodes.get(second.as_ref()).unwrap().rank;

        if first_rank < second_rank {
            self.nodes.get_mut(first.as_ref()).unwrap().parent = second.clone();
        } else if first_rank > second_rank {
            self.nodes.get_mut(second.as_ref()).unwrap().parent = first.clone();
        } else {
            self.nodes.get_mut(second.as_ref()).unwrap().parent = first.clone();
            self.nodes.get_mut(first.as_str()).unwrap().rank += 1;
        }
    }
}

#[derive(Default, Debug)]
struct SuperNode {
    parent: Rc<String>,
    rank: u32,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Edge(String, String);
impl Edge {
    pub fn parse_line(line: &str) -> Vec<Edge> {
        let (name, nodes) = line.split_once(": ").unwrap();

        nodes
            .trim()
            .split_ascii_whitespace()
            .map(|s| Edge(name.to_string(), s.to_string()))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day25::{Day25, Day25P2};

    const EXAMPLE_INPUT: &str = r"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
    #[test]
    fn test_example() {
        assert_eq!(Day25::solve(EXAMPLE_INPUT.lines()), "54")
    }

    #[test]
    #[should_panic]
    fn test_example_p2() {
        assert_eq!(Day25P2::solve(EXAMPLE_INPUT.lines()), "")
    }
}
