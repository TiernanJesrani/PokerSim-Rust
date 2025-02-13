use polars::df;
mod models;
mod tests;
use std::io::{self, Write};
use std::time::Instant;
use std::hint::black_box;

fn main() {
    let start_time = Instant::now();    


    let mut monte = models::monte_model::MonteModel::new(1_000_000);

    monte.run_sim();

    let elapsed = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed);
}