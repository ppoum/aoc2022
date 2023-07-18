struct Procedure {
    pub quantity: usize,
    pub origin: usize,
    pub destination: usize,
}

impl From<&String> for Procedure {
    fn from(item: &String) -> Self {
        // String should have format: move x from y to z
        let mut words = item.split(' ');
        let quantity = words
            .nth(1)
            .expect("Invalid Procedure string.")
            .parse::<usize>()
            .expect("Invalid Procedure string.");
        let origin = words
            .nth(1)
            .expect("Invalid Procedure string.")
            .parse::<usize>()
            .expect("Invalid Procedure string.");
        let destination = words
            .nth(1)
            .expect("Invalid Procedure string.")
            .parse::<usize>()
            .expect("Invalid Procedure string.");

        Procedure {
            quantity,
            origin,
            destination,
        }
    }
}

pub fn part1(lines: Vec<String>) -> String {
    let mut stacks = parse_stacks(&lines);
    let procedures = parse_procedures(&lines);

    for proc in procedures {
        for _ in 0..proc.quantity {
            let item = stacks[proc.origin - 1].pop().unwrap();
            stacks[proc.destination - 1].push(item);
        }
    }

    // Pop each stack to get answer
    let mut answer = String::new();
    for mut stack in stacks {
        answer.push(stack.pop().unwrap());
    }
    answer
}

pub fn part2(lines: Vec<String>) -> String {
    let mut stacks = parse_stacks(&lines);
    let procedures = parse_procedures(&lines);

    for proc in procedures {
        let item = stacks[proc.origin - 1].pop().unwrap();
        stacks[proc.destination - 1].push(item);
        let dest_n = stacks[proc.destination - 1].len();
        for _ in 1..proc.quantity {
            // Insert at "last index" (not refreshed), causing new items to get pushed to the right
            let item = stacks[proc.origin - 1].pop().unwrap();
            stacks[proc.destination - 1].insert(dest_n - 1, item);
        }
    }

    // Pop each stack to get answer
    let mut answer = String::new();
    for mut stack in stacks {
        answer.push(stack.pop().unwrap());
    }
    answer
}

fn parse_stacks(lines: &[String]) -> Vec<Vec<char>> {
    // Get number of stacks from line length
    // 3 chars per stack, +1 space between each stacks
    // 4n-1 = length, n = (length + 1) / 4
    let n = (lines[0].len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(n);

    for _ in 0..n {
        stacks.push(Vec::new());
    }
    for line in lines {
        if line.starts_with(" 1 ") {
            // End of stacks
            break;
        }
        let items = line
            .as_bytes()
            .chunks(4)
            .map(|b| unsafe { std::str::from_utf8_unchecked(b) })
            .collect::<Vec<&str>>();

        for (i, &item) in items.iter().enumerate() {
            // Item = [C], extract character from brackets
            let c = item.chars().nth(1).unwrap();
            if c != ' ' {
                // Don't push if empty character (no crate)
                stacks[i].insert(0, c);
            }
        }
    }

    stacks
}

fn parse_procedures(lines: &[String]) -> Vec<Procedure> {
    // Skip lines until we find the empty line (signifies end of stack definition, start of
    // procedures)
    let lines_iter = lines.iter().skip_while(|&s| !s.is_empty()).skip(1);
    let mut procedures: Vec<Procedure> = Vec::new();

    // Convert line to procedure struct
    for line in lines_iter {
        procedures.push(Procedure::from(line));
    }
    procedures
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = [
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(part1(data), "CMZ");
    }

    #[test]
    fn test_part2() {
        let data = [
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(part2(data), "MCD");
    }
}
