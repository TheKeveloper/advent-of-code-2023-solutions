use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;

use crate::common::Solution;

pub enum Day19 {}

impl Solution for Day19 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let puzzle = Puzzle::from_lines(lines);
        puzzle
            .parts
            .iter()
            .filter(|part| puzzle.get_final_accepted(part))
            .map(|part| part.ratings.values().sum::<i64>())
            .sum::<i64>()
            .to_string()
    }
}

pub enum Day19P2 {}
impl Solution for Day19P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let puzzle = Puzzle::from_lines(lines);
        todo!()
    }
}

struct Puzzle {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Puzzle {
    pub fn from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Puzzle {
        let combined = lines.map(|s| s.as_ref().to_string()).join("\n");
        let (workflows, parts) = combined.split_once("\n\n").unwrap();
        Puzzle {
            workflows: workflows
                .lines()
                .map(|line| line.parse::<Workflow>().unwrap())
                .map(|workflow| (workflow.name.to_string(), workflow))
                .collect(),
            parts: parts.lines().map(|line| line.parse().unwrap()).collect(),
        }
    }

    fn get_final_accepted(&self, part: &Part) -> bool {
        let mut outcome: &Outcome = self.workflows.get("in").unwrap().get_outcome(part);

        loop {
            match outcome {
                Outcome::Accepted => {
                    return true;
                }
                Outcome::Rejected => {
                    return false;
                }
                Outcome::Next(next) => {
                    outcome = self.workflows.get(next).unwrap().get_outcome(part);
                }
            }
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default_outcome: Outcome,
}

impl Workflow {
    pub fn get_outcome(&self, part: &Part) -> &Outcome {
        self.rules
            .iter()
            .find(|rule| rule.satisfies(part))
            .map(|rule| &rule.outcome)
            .unwrap_or_else(|| &self.default_outcome)
    }
}

impl FromStr for Workflow {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (name, rest) = line.split_once('{').ok_or_else(|| {
            anyhow::Error::msg("Failed to parse workflow").context(line.to_string())
        })?;
        let rest = rest.trim_end_matches('}');
        let (rules, default_outcome) = rest.rsplit_once(',').ok_or_else(|| {
            anyhow::Error::msg("Failed to parse workflow").context(line.to_string())
        })?;
        Ok(Workflow {
            name: name.to_string(),
            rules: rules
                .split(',')
                .map(|s| s.parse())
                .collect::<Result<Vec<_>, anyhow::Error>>()?,
            default_outcome: default_outcome.parse()?,
        })
    }
}

struct Rule {
    category: Category,
    condition: Condition,
    threshold: i64,
    outcome: Outcome,
}

impl Rule {
    pub fn satisfies(&self, part: &Part) -> bool {
        let part_rating = part.get_rating(&self.category);
        match self.condition {
            Condition::GreaterThan => part_rating > self.threshold,
            Condition::LessThan => part_rating < self.threshold,
        }
    }

    /// returns two ranges, the first satisfying, and the second not
    pub fn split_satisfying(&self, part_range: &PartRange) -> (PartRange, PartRange) {
        let Range { min, max } = *part_range.get_range(&self.category);
        let (satisfying_range, other_range) = match self.condition {
            Condition::GreaterThan => (
                Range {
                    min: std::cmp::max(self.threshold + 1, min),
                    max,
                },
                Range {
                    min,
                    max: std::cmp::min(self.threshold, max),
                },
            ),
            Condition::LessThan => (
                Range {
                    min,
                    max: std::cmp::min(self.threshold - 1, max),
                },
                Range {
                    min: std::cmp::max(self.threshold, min),
                    max,
                },
            ),
        };

        let mut satisfying = part_range.clone();
        satisfying.ratings.insert(self.category, satisfying_range);
        let mut other = part_range.clone();
        other.ratings.insert(self.category, other_range);

        (satisfying, other)
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (category, rest) = s.split_at(1);
        let (condition, rest) = rest.split_at(1);
        let (threshold, outcome) = rest.split_once(':').ok_or_else(|| {
            anyhow::Error::msg("Could not parse outcome and threshold").context(s.to_string())
        })?;
        Ok(Rule {
            category: category.parse()?,
            condition: condition.parse()?,
            threshold: threshold.parse()?,
            outcome: outcome.parse()?,
        })
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
enum Outcome {
    Accepted,
    Rejected,
    Next(String),
}

impl FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "R" => Outcome::Rejected,
            "A" => Outcome::Accepted,
            other => Outcome::Next(other.to_string()),
        })
    }
}

#[derive(Clone, Eq, PartialEq)]
struct PartRange {
    ratings: HashMap<Category, Range>,
}

impl PartRange {
    pub fn get_range(&self, category: &Category) -> &Range {
        self.ratings.get(category).unwrap()
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Range {
    min: i64,
    max: i64,
}

impl Range {
    pub fn min(&self) -> i64 {
        self.min
    }

    pub fn max(&self) -> i64 {
        self.max
    }

    pub fn is_empty(&self) -> bool {
        self.max <= self.min
    }
}

struct Part {
    ratings: HashMap<Category, i64>,
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = line.trim_start_matches('{').trim_end_matches('}');
        Ok(Part {
            ratings: line
                .split(',')
                .map(|s| {
                    let (category, value) = s.split_once('=').ok_or_else(|| {
                        anyhow::Error::msg("Failed to parse ratings").context(s.to_string())
                    })?;
                    Ok((category.parse::<Category>()?, value.parse::<i64>()?))
                })
                .collect::<Result<HashMap<_, _>, anyhow::Error>>()?,
        })
    }
}

impl Part {
    pub fn get_rating(&self, category: &Category) -> i64 {
        *self.ratings.get(category).unwrap()
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

impl FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "x" => Ok(Category::X),
            "m" => Ok(Category::M),
            "a" => Ok(Category::A),
            "s" => Ok(Category::S),
            _ => Err(
                anyhow::Error::msg("Attempted to convert invalid string to category")
                    .context(s.to_string()),
            ),
        }
    }
}

enum Condition {
    GreaterThan,
    LessThan,
}

impl FromStr for Condition {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Condition::GreaterThan),
            "<" => Ok(Condition::LessThan),
            _ => {
                Err(anyhow::Error::msg("Could not parse invalid condition").context(s.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day19::{Day19, Day19P2};

    const EXAMPLE_INPUT: &str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    #[test]
    fn test_example() {
        assert_eq!(Day19::solve(EXAMPLE_INPUT.lines()), "19114")
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(Day19P2::solve(EXAMPLE_INPUT.lines()), "167409079868000")
    }
}
