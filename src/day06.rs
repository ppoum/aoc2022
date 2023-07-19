use std::collections::VecDeque;

pub fn part1(lines: Vec<String>) -> usize {
    get_unique_slice_n(&lines[0], 4)
}

pub fn part2(lines: Vec<String>) -> usize {
    get_unique_slice_n(&lines[0], 14)
}

fn get_unique_slice_n(packet: &str, length: usize) -> usize {
    let mut queue: VecDeque<char> = VecDeque::with_capacity(length);
    let mut counters = [0_u32; 26];
    let mut duplicate_cnt = 0; // Counts how many characters are duplicates (AABC has 1, since the
                               // 2nd A is a duplicate, AAAB has 2 (2nd & 3rd A))

    // Initialize window with first n chars
    for char in packet.chars().take(length) {
        let char_idx = char as usize - 'a' as usize;
        queue.push_front(char);
        counters[char_idx] += 1; // Add char to window
        if counters[char_idx] > 1 {
            // Duplicated
            duplicate_cnt += 1;
        }
    }

    for (i, char) in packet.chars().enumerate().skip(length) {
        // Check if condition is met
        if duplicate_cnt == 0 {
            return i;
        }

        // Remove outgoing character
        let out_char = queue.pop_back().unwrap();
        let out_idx = out_char as usize - 'a' as usize;
        counters[out_idx] -= 1;
        if counters[out_idx] >= 1 {
            // Character was present more than once before, 1 less duplicate
            duplicate_cnt -= 1;
        }

        let char_idx = char as usize - 'a' as usize;
        queue.push_front(char);
        counters[char_idx] += 1;
        if counters[char_idx] > 1 {
            // Duplicated
            duplicate_cnt += 1;
        }
    }

    panic!("Not found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = vec![String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")];
        assert_eq!(part1(data), 5);
    }

    #[test]
    fn test_part2() {
        let data = vec![String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")];
        assert_eq!(part2(data), 19);
    }
}
