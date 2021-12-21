use crate::models::{Game, PawnPositions, QuanticGame};

mod models;

fn main() {
    let input = PawnPositions::new(10, 4);
    let mut game = Game::new(input);

    println!("part 1: {}", game.play());

    let mut game = QuanticGame::default();
    println!("part 2: {}", game.play(input));
}
