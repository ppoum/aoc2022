use std::cmp;

pub fn part1(lines: Vec<String>) -> u32 {
    let mut max_cal = 0;
    let mut curr_max = 0;
    for line in lines {
        match line.as_str() {
            "" => {
                // Change to new elf, reset curr_max
                max_cal = cmp::max(max_cal, curr_max);
                curr_max = 0;
            }
            str => {
                // Non empty string, check if integer
                match str.parse::<u32>() {
                    Ok(n) => curr_max += n,
                    Err(e) => panic!("Invalid file format. {}", e),
                }
            }
        }
    }
    cmp::max(max_cal, curr_max)
}

pub fn part2(lines: Vec<String>) -> u32 {
    let mut elves = Vec::with_capacity(lines.len());

    let mut curr_elf = 0;
    for line in lines {
        match line.as_str() {
            "" => {
                elves.push(curr_elf);
                curr_elf = 0;
            }
            str => {
                curr_elf += str.parse::<u32>().expect("Invalid file format.");
            }
        }
    }
    elves.push(curr_elf);
    elves.sort();
    elves.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let data: Vec<String> = [
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(part1(data), 24000);
    }

    #[test]
    fn test_part2() {
        let data: Vec<String> = [
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(part2(data), 45000);
    }
}
