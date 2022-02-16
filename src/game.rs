
#[derive(Debug, PartialEq)]
pub enum Result {
    Win,
    Tie,
    Lose,
}

#[derive(Debug, PartialEq)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
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