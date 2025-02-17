mod models;
mod tests;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();    

    let card_1 = std::env::args().nth(1).expect("no card_1 given");
    let card_2 = std::env::args().nth(2).expect("no card_2 given");
    let num_players = std::env::args().nth(3).expect("no players specified");

    //println!("Hello");
    //println!("card 1: {:?}, card 2: {:?}", card_1.chars().nth(0).expect("empty arg"), card_2);
    //let mut monte_sim = models::monte_model::MonteModel::new(250_000);
    
    let cli = models::cli_model::Cli::new(&card_1, &card_2, &num_players);
    //monte_sim.run_sim();

    let elapsed = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed);
}