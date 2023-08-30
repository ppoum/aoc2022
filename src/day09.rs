use std::collections::HashSet;

pub fn part1(lines: Vec<String>) -> usize {
    let mut h_pos = (0isize, 0isize);
    let mut t_pos = (0isize, 0isize);
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert(t_pos);
    for line in lines {
        let mut iter = line.split(' ');
        let dir = iter.next().unwrap();
        let n: usize = iter.next().unwrap().parse().unwrap();

        for _ in 0..n {
            // Move head based on input, then move tail if needed
            match dir {
                "U" => h_pos.1 += 1,
                "D" => h_pos.1 -= 1,
                "L" => h_pos.0 -= 1,
                "R" => h_pos.0 += 1,
                _ => unreachable!(),
            }

            if h_pos.1 == t_pos.1 && h_pos.0.abs_diff(t_pos.0) >= 2 {
                // Same y, too far in x
                if h_pos.0 > t_pos.0 {
                    // Move tail right
                    t_pos.0 += 1;
                } else {
                    // Move tail left
                    t_pos.0 -= 1;
                }
            } else if h_pos.0 == t_pos.0 && h_pos.1.abs_diff(t_pos.1) >= 2 {
                // Same x, too far in y
                if h_pos.1 > t_pos.1 {
                    // Move up
                    t_pos.1 += 1;
                } else {
                    t_pos.1 -= 1;
                }
            } else if h_pos.0.abs_diff(t_pos.0) >= 2 || h_pos.1.abs_diff(t_pos.1) >= 2 {
                // Non-touching diagonal, move both x & y in correct direction
                if h_pos.0 > t_pos.0 {
                    t_pos.0 += 1;
                } else {
                    t_pos.0 -= 1;
                }
                if h_pos.1 > t_pos.1 {
                    t_pos.1 += 1;
                } else {
                    t_pos.1 -= 1;
                }
            }

            visited.insert(t_pos);
        }
    }
    visited.len()
}

// PART 2 functions (could be used to solve part1 as well if wanted)
fn move_knot(head: (isize, isize), curr: (isize, isize)) -> (isize, isize) {
    let mut new_pos = curr;
    if head.1 == curr.1 && head.0.abs_diff(curr.0) >= 2 {
        // Same y, too far in x
        if head.0 > curr.0 {
            // Move tail right
            new_pos.0 += 1;
        } else {
            // Move tail left
            new_pos.0 -= 1;
        }
    } else if head.0 == curr.0 && head.1.abs_diff(curr.1) >= 2 {
        // Same x, too far in y
        if head.1 > curr.1 {
            // Move up
            new_pos.1 += 1;
        } else {
            new_pos.1 -= 1;
        }
    } else if head.0.abs_diff(curr.0) >= 2 || head.1.abs_diff(curr.1) >= 2 {
        // Non-touching diagonal, move both x & y in correct direction
        if head.0 > curr.0 {
            new_pos.0 += 1;
        } else {
            new_pos.0 -= 1;
        }
        if head.1 > curr.1 {
            new_pos.1 += 1;
        } else {
            new_pos.1 -= 1;
        }
    }
    new_pos
}
pub fn part2(lines: Vec<String>) -> usize {
    let mut knots = [(0, 0); 10];
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert(knots[9]);

    for line in lines {
        let mut iter = line.split(' ');
        let dir = iter.next().unwrap();
        let n: usize = iter.next().unwrap().parse().unwrap();

        for _ in 0..n {
            // Move head (0th knot) from input
            match dir {
                "U" => knots[0].1 += 1,
                "D" => knots[0].1 -= 1,
                "L" => knots[0].0 -= 1,
                "R" => knots[0].0 += 1,
                _ => unreachable!(),
            }

            // Move remaining knots based on previous knot position
            for i in 1..10 {
                knots[i] = move_knot(knots[i - 1], knots[i])
            }

            // Save location of last knot
            visited.insert(knots[9]);
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data: Vec<String> = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
        .split('\n')
        .map(String::from)
        .collect();
        assert_eq!(part1(data), 13);
    }

    #[test]
    fn test_part2() {
        // First example
        let data: Vec<String> = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
        .split('\n')
        .map(String::from)
        .collect();
        assert_eq!(part2(data), 1);

        // Second example
        let data: Vec<String> = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            .split('\n')
            .map(String::from)
            .collect();
        assert_eq!(part2(data), 36);
    }
}
