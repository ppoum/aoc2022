pub fn part1(lines: Vec<String>) -> u32 {
    let mut score = 0;
    for mut pair in lines.iter().map(|s| s.split(',')) {
        let mut r1_str = pair.next().expect("Invalid file format.").split('-');
        let mut r2_str = pair.next().expect("Invalid file format.").split('-');

        let r1_start = r1_str.next().unwrap().parse::<u32>().unwrap();
        let r1_end = r1_str.next().unwrap().parse::<u32>().unwrap();
        let r2_start = r2_str.next().unwrap().parse::<u32>().unwrap();
        let r2_end = r2_str.next().unwrap().parse::<u32>().unwrap();

        // Check if r1 contains r2 or inverse
        if (r1_start <= r2_start && r1_end >= r2_end) || (r2_start <= r1_start && r2_end >= r1_end)
        {
            score += 1;
        }
    }
    score
}

pub fn part2(lines: Vec<String>) -> u32 {
    let mut score = 0;
    for mut pair in lines.iter().map(|s| s.split(',')) {
        let mut r1_str = pair.next().expect("Invalid file format.").split('-');
        let mut r2_str = pair.next().expect("Invalid file format.").split('-');

        let r1_start = r1_str.next().unwrap().parse::<u32>().unwrap();
        let r1_end = r1_str.next().unwrap().parse::<u32>().unwrap();
        let r2_start = r2_str.next().unwrap().parse::<u32>().unwrap();
        let r2_end = r2_str.next().unwrap().parse::<u32>().unwrap();

        // Check for overlap
        if (r1_end >= r2_start && r1_start <= r2_start)
            || (r2_end >= r1_start && r2_start <= r1_start)
        {
            score += 1;
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
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(part1(data), 2);
    }

    #[test]
    fn test_part2() {
        let data = [
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(part2(data), 4);
    }
}
