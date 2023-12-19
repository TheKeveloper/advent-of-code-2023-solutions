use std::str::FromStr;

use crate::common::Solution;

pub enum Day19 {}

impl Solution for Day19 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        panic!(
            "lines: {:?}",
            lines.map(|s| s.as_ref().to_string()).collect::<Vec<_>>()
        )
    }
}

pub enum Day19P2 {}
impl Solution for Day19P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        panic!(
            "lines: {:?}",
            lines.map(|s| s.as_ref().to_string()).collect::<Vec<_>>()
        )
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default_outcome: Outcome,
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

    const EXAMPLE_INPUT: &str = r"";
    #[test]
    #[should_panic]
    fn test_example() {
        assert_eq!(Day19::solve(EXAMPLE_INPUT.lines()), "")
    }

    #[test]
    #[should_panic]
    fn test_example_p2() {
        assert_eq!(Day19P2::solve(EXAMPLE_INPUT.lines()), "")
    }
}
