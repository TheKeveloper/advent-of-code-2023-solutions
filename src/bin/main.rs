use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use structopt::StructOpt;

use advent_of_code_2023_solutions::common::Solution;
use advent_of_code_2023_solutions::day1::{Day1, Day1P2};
use advent_of_code_2023_solutions::day2::Day2;
use advent_of_code_2023_solutions::day3::Day3;

fn main() {
    let cli = Cli::from_args();

    let file = File::open(cli.input).expect("Could not read input");
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|result| result.expect("Could not read line from input file"));
    let result = match (cli.day, cli.part) {
        (1, 1) => Day1::solve(lines),
        (1, 2) => Day1P2::solve(lines),
        (2, 1) => Day2::solve(lines),
        (3, 1) => Day3::solve(lines),
        _ => unimplemented!(),
    };
    println!("{}", result)
}

#[derive(StructOpt)]
struct Cli {
    day: u8,
    part: u8,
    input: PathBuf,
}
