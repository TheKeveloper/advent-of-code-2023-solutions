use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn get_input_path() -> PathBuf {
    PathBuf::from(
        env::args()
            .nth(1)
            .expect("Required one command line argument"),
    )
}

pub fn get_input_lines() -> impl Iterator<Item = String> {
    let file = File::open(get_input_path()).expect("Could not open input path");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|result| result.expect("Could not read line from input file"))
}

pub trait Solution {
    fn default_print_solution() {
        println!("{}", Self::solve(get_input_lines()));
    }

    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> u32;
}
