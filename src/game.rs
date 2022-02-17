
#[derive(Debug, PartialEq)]
pub enum Result {
    Win,
    Tie,
    Lose,
}

impl Result {
    pub fn compute_score(&self, score: Score) -> Score {
        match self {
            Result::Win => Score{
                player: score.player + 1,
                ..score
            },
            Result::Lose => Score{
                computer: score.computer + 1,
                ..score
            },
            Result::Tie => score,
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

pub struct Score {
    pub player: u32,
    pub computer: u32,

}

impl Score {
    pub fn new() -> Score {
        Score {
            player: 0,
            computer: 0,
        }
    }

    pub fn check(&self) -> bool {
        if self.player == 3 || self.computer == 3 {
            return true;
        }
        false
    }

    pub fn print_winner(&self) {
        if self.player == 3 {
            println!("Player wins!");
        } else {
            println!("Computer wins!");
        }
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