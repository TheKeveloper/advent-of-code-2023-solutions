use std::str::FromStr;

use crate::common::Solution;

pub enum Day5 {}

impl Solution for Day5 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let lines: Vec<_> = lines.map(|line| line.as_ref().to_string()).collect();
        let problem = Problem::from_lines(lines.as_slice());
        problem
            .seeds
            .iter()
            .map(|seed| problem.get_location(*seed))
            .min()
            .unwrap()
            .to_string()
    }
}

pub enum Day5P2 {}

impl Solution for Day5P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let lines: Vec<_> = lines.map(|line| line.as_ref().to_string()).collect();
        let problem = Problem::from_lines(lines.as_slice());

        // this is dumb but it's my bedtime.
        // I think we can do something where we find the "endpoints" of all the eventually mapped
        // ranges, but will save that for another time.
        let seed_line = &lines[0];
        let parts: Vec<_> = seed_line
            .strip_prefix("seeds: ")
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        parts
            .as_slice()
            .chunks(2)
            .flat_map(|chunk| (chunk[0]..=(chunk[0] + chunk[1])))
            .map(|seed| problem.get_location(seed))
            .min()
            .unwrap()
            .to_string()
    }
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Problem {
    seeds: Vec<u64>,
    mappings: Vec<Mapping>,
}
impl Problem {
    pub fn from_lines(lines: &[String]) -> Problem {
        let seeds_line = lines[0].as_str();
        let seeds: Vec<u64> = seeds_line
            .strip_prefix("seeds: ")
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let mapping_lines = &lines[2..lines.len()];
        let mappings = mapping_lines
            .split(|s| s.is_empty())
            .map(Mapping::from_lines)
            .collect();

        Problem { seeds, mappings }
    }

    fn get_location(&self, seed: u64) -> u64 {
        self.find_next(seed, "seed")
    }

    fn find_next(&self, value: u64, category: &str) -> u64 {
        if category.eq("location") {
            return value;
        }

        let mapping = self.get_mapping(category);
        self.find_next(mapping.map_value(value), mapping.destination.as_str())
    }

    fn get_mapping(&self, source: &str) -> &Mapping {
        self.mappings
            .iter()
            .find(|mapping| mapping.source.as_str().eq(source))
            .unwrap()
    }
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Mapping {
    source: String,
    destination: String,
    ranges: Vec<MappingRange>,
}

impl Mapping {
    pub fn from_lines(lines: &[String]) -> Mapping {
        let parts = lines[0]
            .strip_suffix(" map:")
            .unwrap()
            .split("-to-")
            .collect::<Vec<_>>();
        let [source, destination] = parts.as_slice() else {
            panic!()
        };
        let mapping_ranges = lines
            .iter()
            .skip(1)
            .map(|s| MappingRange::from_string(s.as_str()))
            .collect();
        Mapping {
            source: source.to_string(),
            destination: destination.to_string(),
            ranges: mapping_ranges,
        }
    }

    pub fn map_value(&self, value: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|range| range.get_mapped_value(value))
            .unwrap_or(value)
    }
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct MappingRange {
    source_start: u64,
    dest_start: u64,
    length: u64,
}

impl MappingRange {
    pub fn from_string(line: &str) -> MappingRange {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        let [dest_start, source_start, length] = parts.as_slice() else {
            panic!()
        };
        MappingRange {
            source_start: source_start.parse().unwrap(),
            dest_start: dest_start.parse().unwrap(),
            length: length.parse().unwrap(),
        }
    }

    pub fn get_mapped_value(&self, input: u64) -> Option<u64> {
        if input < self.source_start {
            return None;
        }
        let dist = input - self.source_start;
        if dist > self.length {
            return None;
        }
        Some(self.dest_start + dist)
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day5::{Day5, Day5P2, Problem};

    const EXAMPLE_INPUT: &'static str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_example() {
        assert_eq!(Day5::solve(EXAMPLE_INPUT.lines()), "35");
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(Day5P2::solve(EXAMPLE_INPUT.lines()), "46");
    }

    #[test]
    fn test_parsing() {
        let problem = Problem::from_lines(
            EXAMPLE_INPUT
                .lines()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .as_slice(),
        );

        assert_eq!(problem.seeds, vec![79, 14, 55, 13]);

        assert_eq!(problem.mappings[0].source, "seed");
        assert_eq!(problem.mappings[0].destination, "soil");
        assert_eq!(problem.mappings[0].ranges[0].source_start, 98);
        assert_eq!(problem.mappings[0].ranges[0].dest_start, 50);
        assert_eq!(problem.mappings[0].ranges[0].length, 2);
    }
}
