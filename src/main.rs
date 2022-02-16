mod game;
use std::io::{stdin,stdout,Write};
use game::*;
fn main_game_loop() {
    let mut human_player_score = 0;
    let mut computer_player_score = 0;
    loop {
        let mut s=String::new();
        println!("Choose your move: Rock, Paper, Scissors");
        let _ = stdin().read_line(&mut s).unwrap();
        let player_1_choice = RPS::from_str(&s);
        match player_1_choice {
            Some(player_1_choice) => {
                let cpu_choice = random_move();
                let result = result(player_1_choice, cpu_choice);
                let (player_1_score, player_2_score) = result.compute_score(human_player_score, computer_player_score);
                human_player_score = player_1_score;
                computer_player_score = player_2_score;
                println!("{:?}", result);
            },
            None => {
                println!("Invalid move");
                continue;
            }
        }

        if human_player_score == 3 || computer_player_score == 3 {
            println!("Game over");
            if human_player_score == 3 {
                println!("You won!");
            } else {
                println!("You lost!");
            }
            break;
            
        }
    }
}

fn main() {
    println!("Welcome to the game!");
    main_game_loop();
    println!("Bye!");
}
