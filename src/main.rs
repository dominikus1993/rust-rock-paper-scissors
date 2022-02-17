mod game;
use std::io::{stdin,stdout,Write};
use game::*;


fn main_game_loop() {
    let mut score = Score::new();
    loop {
        let mut s=String::new();
        println!("Choose your move: Rock, Paper, Scissors");
        let _ = stdin().read_line(&mut s).unwrap();
        let player_1_choice = RPS::from_str(&s);
        match player_1_choice {
            Some(player_1_choice) => {
                let cpu_choice = random_move();
                let result = result(player_1_choice, cpu_choice);
                score = result.compute_score(score);
                println!("{:?}", result);
            },
            None => {
                println!("Invalid move");
                continue;
            }
        }

        if score.check() {
            println!("Game over");
            score.print_winner();
            break;
            
        }
    }
}

fn main() {
    println!("Welcome to the game!");
    main_game_loop();
    println!("Bye!");
}
