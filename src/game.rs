
#[derive(Debug, PartialEq)]
pub enum Result {
    Win,
    Tie,
    Lose,
}

impl Result {
    pub fn compute_score(&self, p1s: i32, p2s: i32) -> (i32, i32) {
        match self {
            Result::Win => (p1s + 1, p2s),
            Result::Lose => (p1s, p2s + 1),
            Result::Tie => (p1s, p2s),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    pub fn from_str(s: &str) -> Option<RPS> {
        let trimmed = s.trim();
        let lower = trimmed.to_lowercase();
        match lower.as_str() {
            "rock" => Some(RPS::Rock),
            "paper" => Some(RPS::Paper),
            "scissors" => Some(RPS::Scissors),
            _ => None,
        }
    }
}

pub fn result(player1: RPS, player2: RPS) -> Result {
    match (player1, player2) {
        (RPS::Rock, RPS::Scissors) => Result::Win,
        (RPS::Rock, RPS::Paper) => Result::Lose,
        (RPS::Paper, RPS::Rock) => Result::Win,
        (RPS::Paper, RPS::Scissors) => Result::Lose,
        (RPS::Scissors, RPS::Rock) => Result::Lose,
        (RPS::Scissors, RPS::Paper) => Result::Win,
        _ => Result::Tie,
    }
}

pub fn random_move() -> RPS {
    match rand::random::<u8>() % 3 {
        0 => RPS::Rock,
        1 => RPS::Paper,
        _ => RPS::Scissors,
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_result() {
        let rock = RPS::Rock;
        let paper = RPS::Paper;
        let subject = result(rock, paper);
        assert_eq!(subject, Result::Lose);
    }
}