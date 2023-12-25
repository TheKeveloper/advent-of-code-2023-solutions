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
use advent_of_code_2023_solutions::day15::{Day15, Day15P2};
use advent_of_code_2023_solutions::day16::{Day16, Day16P2};
use advent_of_code_2023_solutions::day17::{Day17, Day17P2};
use advent_of_code_2023_solutions::day18::{Day18, Day18P2};
use advent_of_code_2023_solutions::day19::{Day19, Day19P2};
use advent_of_code_2023_solutions::day2::{Day2, Day2P2};
use advent_of_code_2023_solutions::day20::{Day20, Day20P2};
use advent_of_code_2023_solutions::day21::{Day21, Day21P2};
use advent_of_code_2023_solutions::day22::{Day22, Day22P2};
use advent_of_code_2023_solutions::day23::{Day23, Day23P2};
use advent_of_code_2023_solutions::day24::{Day24, Day24P2};
use advent_of_code_2023_solutions::day25::{Day25, Day25P2};
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
        (15, 1) => Day15::solve(lines),
        (15, 2) => Day15P2::solve(lines),
        (16, 1) => Day16::solve(lines),
        (16, 2) => Day16P2::solve(lines),
        (17, 1) => Day17::solve(lines),
        (17, 2) => Day17P2::solve(lines),
        (18, 1) => Day18::solve(lines),
        (18, 2) => Day18P2::solve(lines),
        (19, 1) => Day19::solve(lines),
        (19, 2) => Day19P2::solve(lines),
        (20, 1) => Day20::solve(lines),
        (20, 2) => Day20P2::solve(lines),
        (21, 1) => Day21::solve(lines),
        (21, 2) => Day21P2::solve(lines),
        (22, 1) => Day22::solve(lines),
        (22, 2) => Day22P2::solve(lines),
        (23, 1) => Day23::solve(lines),
        (23, 2) => Day23P2::solve(lines),
        (24, 1) => Day24::solve(lines),
        (24, 2) => Day24P2::solve(lines),
        (25, 1) => Day25::solve(lines),
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
