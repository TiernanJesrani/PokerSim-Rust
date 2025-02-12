mod models;
mod tests;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();    
    let mut sum = 0;
    for _i in 0..100_000 {
        let mut game = models::game_model::Game::new(3, false, 0, 0);

        game.deal();

        game.flop();

        game.turn();

        game.river();

        game.form_hand_strengths();

        sum += game.main_wins();
    }
    let elapsed = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!("AA wins: {:?}%", ( sum as f32 / 100_000 as f32))

    // game.deal();

    // game.flop();

    // game.turn();

    // game.river();

    // game.form_hand_strengths();

    // for i in 0..game.hand_strengths.len() {
    //     println!("\n{:?}", game.hand_strengths[i].hand_type);
    //     println!("{:?}", game.hand_strengths[i].cards_involved);
    //     println!("{:?}", game.hand_strengths[i].cards_leftover);
    // }
    // println!("\n{:?}", game.main_hand_strength.hand_type);
    // println!("{:?}", game.main_hand_strength.cards_involved);
    // println!("{:?}", game.main_hand_strength.cards_leftover);

    // println!("{:?}", game.main_wins());
    
}
