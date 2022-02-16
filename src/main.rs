mod game;
use game::*;
fn main() {
    let a = game::game::RPS::Rock;
    println!("My move is {:?}", a);
    let b = game::random_move();
    println!("\nOpponent move is {:?}", b);
    let res = game::result(a, b);
    println!("{:?}", res);
    println!("Hello, world!");
}
