use std::env;

use aoc2022::read_and_parse_file;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.get(0).unwrap().as_str() {
        "1a" => {
            let input = read_and_parse_file("data/day01.txt");
            println!("{}", aoc2022::day01::part1(input));
        }
        "1b" => {
            let input = read_and_parse_file("data/day01.txt");
            println!("{}", aoc2022::day01::part2(input));
        }
        "2a" => {
            let input = read_and_parse_file("data/day02.txt");
            println!("{}", aoc2022::day02::part1(input));
        }
        "2b" => {
            let input = read_and_parse_file("data/day02.txt");
            println!("{}", aoc2022::day02::part2(input));
        }
        "3a" => {
            let input = read_and_parse_file("data/day03.txt");
            println!("{}", aoc2022::day03::part1(input));
        }
        "3b" => {
            let input = read_and_parse_file("data/day03.txt");
            println!("{}", aoc2022::day03::part2(input));
        }
        "4a" => {
            let input = read_and_parse_file("data/day04.txt");
            println!("{}", aoc2022::day04::part1(input));
        }
        "4b" => {
            let input = read_and_parse_file("data/day04.txt");
            println!("{}", aoc2022::day04::part2(input));
        }
        "5a" => {
            let input = read_and_parse_file("data/day05.txt");
            println!("{}", aoc2022::day05::part1(input));
        }
        "5b" => {
            let input = read_and_parse_file("data/day05.txt");
            println!("{}", aoc2022::day05::part2(input));
        }
        "6a" => {
            let input = read_and_parse_file("data/day06.txt");
            println!("{}", aoc2022::day06::part1(input));
        }
        "6b" => {
            let input = read_and_parse_file("data/day06.txt");
            println!("{}", aoc2022::day06::part2(input));
        }
        "7a" => {
            let input = read_and_parse_file("data/day07.txt");
            println!("{}", aoc2022::day07::part1(input));
        }
        "7b" => {
            let input = read_and_parse_file("data/day07.txt");
            println!("{}", aoc2022::day07::part2(input));
        }
        "7aalt" => {
            let input = read_and_parse_file("data/day07.txt");
            println!("{}", aoc2022::day07_alt::part1(input));
        }
        "7balt" => {
            let input = read_and_parse_file("data/day07.txt");
            println!("{}", aoc2022::day07_alt::part2(input));
        }
        "8a" => {
            let input = read_and_parse_file("data/day08.txt");
            println!("{}", aoc2022::day08::part1(input));
        }
        "8b" => {
            let input = read_and_parse_file("data/day08.txt");
            println!("{}", aoc2022::day08::part2(input));
        }
        "9a" => {
            let input = read_and_parse_file("data/day09.txt");
            println!("{}", aoc2022::day09::part1(input));
        }
        "9b" => {
            let input = read_and_parse_file("data/day09.txt");
            println!("{}", aoc2022::day09::part2(input));
        }
        _ => eprintln!("Invalid day specified"),
    }
}
