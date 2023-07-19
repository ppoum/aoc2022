pub fn part1(lines: Vec<String>) -> usize {
    let n = lines.len();
    let m = lines[0].len();

    let mut grid = vec![vec![0; m]; n];
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            grid[row][col] = c.to_digit(10).unwrap();
        }
    }

    // Calculate tallest in every direction for each coord
    let mut memo = vec![vec![0; m]; n];
    // Max height up
    for col in 0..m {
        let mut tallest = 0;
        for row in 0..n {
            // No need to compare in first iter, just write to memo grid
            memo[row][col] = tallest;
            tallest = std::cmp::max(tallest, grid[row][col]);
        }
    }

    // Down
    for col in 0..m {
        let mut tallest = 0;
        for row in (0..n).rev() {
            // If current max height to edge is smaller than memoized value, overwrite
            memo[row][col] = std::cmp::min(tallest, memo[row][col]);
            tallest = std::cmp::max(tallest, grid[row][col]);
        }
    }

    // Left
    for row in 0..n {
        let mut tallest = 0;
        for col in 0..m {
            memo[row][col] = std::cmp::min(tallest, memo[row][col]);
            tallest = std::cmp::max(tallest, grid[row][col]);
        }
    }

    // Right
    for row in 0..n {
        let mut tallest = 0;
        for col in (0..m).rev() {
            memo[row][col] = std::cmp::min(tallest, memo[row][col]);
            tallest = std::cmp::max(tallest, grid[row][col]);
        }
    }

    // Initialize variable such that we count every tree on the edge as visible
    let mut visible_cnt = 2 * n + 2 * m - 4;

    // Iterate over trees not on the edge
    for row in 1..n - 1 {
        for col in 1..m - 1 {
            if grid[row][col] > memo[row][col] {
                visible_cnt += 1;
            }
        }
    }

    visible_cnt
}

pub fn part2(lines: Vec<String>) -> usize {
    let n = lines.len();
    let m = lines[0].len();

    let mut grid = vec![vec![0; m]; n];
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            grid[row][col] = c.to_digit(10).unwrap();
        }
    }

    // Ignore all positions on edge, as they will get a score multiplied by 0
    let mut max_score = 0;
    for row in 1..n - 1 {
        for col in 1..m - 1 {
            let mut score = 1;
            let coord_height = grid[row][col];

            let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for dir in directions {
                let mut pos = (row as isize, col as isize);
                pos.0 += dir.0;
                pos.1 += dir.1;
                let mut distance = 1;
                while (0..n).contains(&(pos.0 as usize)) && (0..m).contains(&(pos.1 as usize)) {
                    if grid[pos.0 as usize][pos.1 as usize] < coord_height {
                        distance += 1;
                        pos.0 += dir.0;
                        pos.1 += dir.1;
                    } else {
                        // If pos goes OOB, distance still gets increased wrongly, increment
                        // distance here too so that we can decrement it after while loop.
                        distance += 1;
                        break;
                    }
                }
                score *= distance - 1;
            }
            max_score = std::cmp::max(max_score, score);
        }
    }

    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = ["30373", "25512", "65332", "33549", "35390"]
            .map(String::from)
            .to_vec();
        assert_eq!(part1(data), 21);
    }

    #[test]
    fn test_part2() {
        let data = ["30373", "25512", "65332", "33549", "35390"]
            .map(String::from)
            .to_vec();
        assert_eq!(part2(data), 8);
    }
}
