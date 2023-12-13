use std::collections::HashMap;
use std::str::FromStr;

use crate::common::Solution;

pub enum Day12 {}

impl Solution for Day12 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        lines
            .map(|line| line.as_ref().parse::<Record>().unwrap().get_arrangements())
            .sum::<usize>()
            .to_string()
    }
}

struct Record {
    springs: Vec<Condition>,
    damaged_records: Vec<usize>,
}

impl Record {
    pub fn get_arrangements(&self) -> usize {
        let value = self.get_arrangements_dp(&mut HashMap::new(), CacheIndex::default());
        println!("value: {}", value);
        value
    }

    fn get_arrangements_dp(
        &self,
        dp: &mut HashMap<CacheIndex, usize>,
        cache_index: CacheIndex,
    ) -> usize {
        let entry = dp.get(&cache_index).cloned();
        match entry {
            Some(value) => value,
            None => {
                let Some(len) = self.damaged_records.get(cache_index.damaged_index) else {
                    let result = {
                        // either we used up everything, or all the remainders are not damaged
                        if cache_index.springs_index >= self.springs.len()
                            || self.springs[cache_index.springs_index..self.springs.len()]
                                .iter()
                                .all(|val| !val.is_damaged())
                        {
                            1
                        } else {
                            0
                        }
                    };
                    dp.insert(cache_index, result);
                    return result;
                };
                let start = cache_index.springs_index;
                let end = start + len - 1;
                if end >= self.springs.len() {
                    return 0;
                }

                let take_result = if self.springs[start..=end]
                    .iter()
                    .all(|condition| condition.could_be_damaged())
                    // check that the next one after this is is not damaged or this is the last
                    // element in the list
                    && self
                        .springs
                        .get(end + 1)
                        .map(|val| !val.is_damaged())
                        .unwrap_or(true)
                {
                    self.get_arrangements_dp(
                        dp,
                        CacheIndex {
                            springs_index: end + 2,
                            damaged_index: cache_index.damaged_index + 1,
                        },
                    )
                } else {
                    0usize
                };

                let no_take_result = self.get_arrangements_dp(
                    dp,
                    CacheIndex {
                        springs_index: start + 1,
                        damaged_index: cache_index.damaged_index,
                    },
                );

                let result = take_result + no_take_result;
                dp.insert(cache_index, result);
                result
            }
        }
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
struct CacheIndex {
    springs_index: usize,
    damaged_index: usize,
}

impl FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, reports) = s
            .split_once(' ')
            .ok_or(anyhow::Error::msg("Received bad input").context(s.to_string()))?;

        Ok(Record {
            springs: springs.chars().map(|c| c.into()).collect(),
            damaged_records: reports.split(',').map(|val| val.parse().unwrap()).collect(),
        })
    }
}

enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Condition {
    pub fn could_be_damaged(&self) -> bool {
        matches!(self, Condition::Damaged | Condition::Unknown)
    }

    pub fn is_damaged(&self) -> bool {
        matches!(self, Condition::Damaged)
    }
}

impl From<char> for Condition {
    fn from(value: char) -> Self {
        match value {
            '.' => Condition::Operational,
            '#' => Condition::Damaged,
            '?' => Condition::Unknown,
            _ => panic!("Unexpected value received: {}", value),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day12::Day12;

    const EXAMPLE_INPUT: &'static str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
    #[test]
    fn test_example() {
        assert_eq!(Day12::solve(EXAMPLE_INPUT.lines()), "21")
    }
}
