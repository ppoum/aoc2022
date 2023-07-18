use std::collections::HashSet;

pub fn part1(lines: Vec<String>) -> u32 {
    // Find the one character that is present in both halves of the line
    let mut score = 0;
    for rucksack in lines {
        // Assume string always has even length
        let size = rucksack.len();
        let (h1, h2) = rucksack.split_at(size / 2);
        let mut set: HashSet<char> = HashSet::new();

        // Populate set
        for char in h1.chars() {
            set.insert(char);
        }

        for char in h2.chars() {
            if set.contains(&char) {
                // Convert char to priority (a-z=1-26, A-Z=27-52)
                score += if char.is_ascii_lowercase() {
                    // Lowercase
                    char as u32 - 'a' as u32 + 1
                } else {
                    // Uppercase
                    char as u32 - 'A' as u32 + 27
                };
                break; // As per definition of problem, only 1 char can be in both halves
            }
        }
    }
    score
}

pub fn part2(lines: Vec<String>) -> u32 {
    // Find char present in 3 rows
    let mut score = 0;
    for group in lines.chunks(3) {
        let r1 = &group[0];
        let r2 = &group[1];
        let r3 = &group[2];

        let mut setr1: HashSet<char> = HashSet::new();
        let mut setr2: HashSet<char> = HashSet::new();

        for char in r1.chars() {
            setr1.insert(char);
        }
        for char in r2.chars() {
            setr2.insert(char);
        }
        let intersection = setr1.intersection(&setr2);

        for char in intersection {
            if r3.contains(*char) {
                // Badge character, get integer value from char
                score += if char.is_ascii_lowercase() {
                    // Lowercase
                    *char as u32 - 'a' as u32 + 1
                } else {
                    // Uppercase
                    *char as u32 - 'A' as u32 + 27
                };
                break;
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(part1(data), 157);
    }

    #[test]
    fn test_part2() {
        let data = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(part2(data), 70);
    }
}
