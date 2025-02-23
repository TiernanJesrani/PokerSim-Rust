mod models;
mod tests;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();    

    let args: Vec<String> = std::env::args().collect();

    let card_1 = args.get(1).map(String::as_str).unwrap_or("default");
    let card_2 = args.get(2).map(String::as_str).unwrap_or("default");
    let num_players = args.get(3).map(String::as_str).unwrap_or("default");

    let cards: Vec<Option<String>> = if args.len() > 4 {
        args.iter().skip(4).map(|s| Some(s.clone())).collect()
    } else {
        Vec::new()
    };

    let mut monte_sim = models::monte_model::MonteModel::new(1_000_000);

    if std::env::args().len() < 2 {
        monte_sim.run_sim();
    }
    else {
        let cli = models::cli_model::Cli::new(&card_1, &card_2, &num_players, cards);
        monte_sim.run_one_hand(cli.hand[0], cli.hand[1], cli.num_players, Some(cli.board));
    }

    let elapsed = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed);
}