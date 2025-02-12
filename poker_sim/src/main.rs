mod models;
mod tests;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();    
    let mut sum = 0;

    println!("Main Hand: {:?}", (1, 1));
    for _i in 0..1_000_000 {
        let mut game = models::game_model::Game::new(1, false, 0, 0);

        game.deal();

        game.flop();

        game.turn();

        game.river();

        game.form_hand_strengths();

        let mut wins = game.main_wins();

        // println!("\nMain's hand: {:?}", game.main_hand_strength.seven_cards);
        // println!("Player's hand: {:?}", game.hand_strengths[0].seven_cards);

        // println!("Wins: {:?}", wins);

        sum += wins;

    }
    let elapsed = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!("Wins: {:?}%", ( sum as f32 / 1_000_000 as f32))
}
