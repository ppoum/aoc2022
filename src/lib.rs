use std::fs;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day07_alt;

pub fn read_and_parse_file(path: &str) -> Vec<String> {
    // Reads a file and returns Line iterator
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
