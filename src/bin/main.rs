use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use structopt::StructOpt;

use advent_of_code_2023_solutions::common::Solution;
use advent_of_code_2023_solutions::day1::{Day1, Day1P2};
use advent_of_code_2023_solutions::day10::{Day10, Day10P2};
use advent_of_code_2023_solutions::day11::{Day11, Day11P2};
use advent_of_code_2023_solutions::day12::{Day12, Day12P2};
use advent_of_code_2023_solutions::day13::{Day13, Day13P2};
use advent_of_code_2023_solutions::day14::{Day14, Day14P2};
use advent_of_code_2023_solutions::day2::{Day2, Day2P2};
use advent_of_code_2023_solutions::day3::{Day3, Day3P2};
use advent_of_code_2023_solutions::day4::{Day4, Day4P2};
use advent_of_code_2023_solutions::day5::{Day5, Day5P2};
use advent_of_code_2023_solutions::day6::{Day6, Day6P2};
use advent_of_code_2023_solutions::day7::{Day7, Day7P2};
use advent_of_code_2023_solutions::day8::{Day8, Day8P2};
use advent_of_code_2023_solutions::day9::{Day9, Day9P2};

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
        (2, 2) => Day2P2::solve(lines),
        (3, 1) => Day3::solve(lines),
        (3, 2) => Day3P2::solve(lines),
        (4, 1) => Day4::solve(lines),
        (4, 2) => Day4P2::solve(lines),
        (5, 1) => Day5::solve(lines),
        (5, 2) => Day5P2::solve(lines),
        (6, 1) => Day6::solve(lines),
        (6, 2) => Day6P2::solve(lines),
        (7, 1) => Day7::solve(lines),
        (7, 2) => Day7P2::solve(lines),
        (8, 1) => Day8::solve(lines),
        (8, 2) => Day8P2::solve(lines),
        (9, 1) => Day9::solve(lines),
        (9, 2) => Day9P2::solve(lines),
        (10, 1) => Day10::solve(lines),
        (10, 2) => Day10P2::solve(lines),
        (11, 1) => Day11::solve(lines),
        (11, 2) => Day11P2::solve(lines),
        (12, 1) => Day12::solve(lines),
        (12, 2) => Day12P2::solve(lines),
        (13, 1) => Day13::solve(lines),
        (13, 2) => Day13P2::solve(lines),
        (14, 1) => Day14::solve(lines),
        (14, 2) => Day14P2::solve(lines),
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
