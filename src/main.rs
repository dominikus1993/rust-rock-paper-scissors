use rand::Rng;

#[derive(Debug)]
enum Result {
    Win,
    Tie,
    Lose
}

#[derive(Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

fn result(player1: RPS, player2: RPS) -> Result {
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

fn random_move() -> RPS {
    match rand::random::<u8>() % 3 {
        0 => RPS::Rock,
        1 => RPS::Paper,
        _ => RPS::Scissors,
    }
}

fn main() {
    let a = RPS::Rock;
    print!("My move is {}", a);
    let b = random_move();
    print!("\nOpponent move is {}", b);
    let res = result(a, b);
    println!("{:?}", res);
    println!("Hello, world!");
}
