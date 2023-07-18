#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn new(c: char) -> Self {
        match c {
            'A' | 'X' => RPS::Rock,
            'B' | 'Y' => RPS::Paper,
            'C' | 'Z' => RPS::Scissors,
            c => panic!("Invalid RPS character: {}", c),
        }
    }

    fn result(&self, rhs: RPS) -> u32 {
        match self {
            RPS::Rock => match rhs {
                RPS::Rock => 3,
                RPS::Paper => 0,
                RPS::Scissors => 6,
            },
            RPS::Paper => match rhs {
                RPS::Rock => 6,
                RPS::Paper => 3,
                RPS::Scissors => 0,
            },
            RPS::Scissors => match rhs {
                RPS::Rock => 0,
                RPS::Paper => 6,
                RPS::Scissors => 3,
            },
        }
    }

    fn needed_for_result(other: RPS, result: char) -> Self {
        match result {
            'X' => {
                // Lose to other
                match other {
                    RPS::Rock => RPS::Scissors,
                    RPS::Paper => RPS::Rock,
                    RPS::Scissors => RPS::Paper,
                }
            }
            'Y' => {
                // Draw
                other
            }
            'Z' => {
                // Win
                match other {
                    RPS::Rock => RPS::Paper,
                    RPS::Paper => RPS::Scissors,
                    RPS::Scissors => RPS::Rock,
                }
            }
            c => panic!("Invalid result for RPS: {}", c),
        }
    }
}

pub fn part1(lines: Vec<String>) -> u32 {
    let mut score = 0;
    for mut line in lines.iter().map(|s| s.chars()) {
        let opp_pick = line.next().expect("Invalid file format.");
        let player_pick = line.nth(1).expect("Invalid file format.");

        // Increase score based on player pick (X=1, Y=2, Z=3)
        score += player_pick as u32 - 'W' as u32;

        // Increase score based on match result (win=6, draw=3, lose=0)
        score += RPS::new(player_pick).result(RPS::new(opp_pick));
    }
    score
}

pub fn part2(lines: Vec<String>) -> u32 {
    let mut score = 0;
    for mut line in lines.iter().map(|s| s.chars()) {
        let opp_pick = line.next().expect("Invalid file format.");
        let expected_res = line.nth(1).expect("Invalid file format.");

        // Increase score based on result (X=0, Y=3, Z=6)
        score += match expected_res {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!("Invalid file format."),
        };

        // Increase based on player pick (Rock=1, Paper=2, Scissors=3)
        score += match RPS::needed_for_result(RPS::new(opp_pick), expected_res) {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = ["A Y", "B X", "C Z"].map(String::from).to_vec();
        assert_eq!(part1(data), 15);
    }

    #[test]
    fn test_part2() {
        let data = ["A Y", "B X", "C Z"].map(String::from).to_vec();
        assert_eq!(part2(data), 12);
    }
}
