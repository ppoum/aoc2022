# aoc2022
AOC 2022 solutions in rust

Still a work in progress, not all solutions available.

## Setup
1. Create a `data` folder in the working directory.
2. Inside that folder, save your puzzle inputs under files named `dayXX.txt`, where `XX` is the day number (ex: `day25.txt` or `day05.txt`)

## How to run
`cargo test` will run the program on every example given in the puzzle descriptions. To run the program on your actual puzzle data, first see the setup section. `cargo r XX` or `./executable XX` will run the program for a specific day. `XX` needs to be formatted as `[0-9](a|b)` (ex: `cargo r 5b` runs the fifth's day part 2 puzzle).
