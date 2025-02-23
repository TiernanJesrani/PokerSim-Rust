use crate::models::game_model::Game;
use crate::models::card_model::Card;
use polars::df;
use std::fs::File;
use std::io::Write;
use polars::prelude::*;

pub struct MonteModel {
    estimates: usize,
    conversion_string: [&'static str; 13],
    player_vecs: Vec<Vec<f64>>
}

impl MonteModel {
    pub fn new(estimates: usize) -> MonteModel {
        let player_vecs = vec![
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new()
        ];

        MonteModel { estimates: estimates, conversion_string: ["A", "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K"], player_vecs: player_vecs}
    }

    pub fn run_one_hand(&mut self, card_1: Card, card_2: Card, num_players: usize, additional_cards: Option<Vec<Card>>) -> f64 {
        let mut sum = 0;
        let mut game = Game::new(num_players - 1, card_1, card_2, additional_cards);
        let mut cli = true;
        for _i in 0..self.estimates {
            game.deal();

            if game.board.is_empty() {
                cli = false;
                game.flop();
                game.turn();
                game.river();
            }
            else if game.board.len() < 4 {
                game.turn();
                game.river();
            }
            else if game.board.len() < 5 {
                game.river();
            }

            game.form_hand_strengths();

            sum += game.main_wins();

            game.reset_game();
        }
        if cli {
            println!("{:?}", sum as f64 / (self.estimates / 100) as f64);
        }

        sum as f64 / (self.estimates / 100) as f64
    }

    pub fn run_sim(&mut self) -> () {
        let mut starting_hand = Vec::new();

        for i in 0..13 {
            for j in i..13 {
                let hand_name_o = self.conversion_string[i].to_owned() + self.conversion_string[j] + "o";
                starting_hand.push(hand_name_o);
                for k in 2..10 {
                    let hand_result = (self.run_one_hand(Card::new(0, i), Card::new(1, j), k, None) * 100.0).round() / 100.0;
                    self.player_vecs[k - 2].push(hand_result);
                }
                
                if i != j {
                    let hand_name_s = self.conversion_string[i].to_owned() + self.conversion_string[j] + "s";
                    starting_hand.push(hand_name_s);
                    for k in 2..10 {
                        let hand_result = self.run_one_hand(Card::new(0, i), Card::new(0, j), k, None);
                        self.player_vecs[k - 2].push((hand_result * 100.0).round() / 100.0);
                    }
                }
            }
        }

        let mut df = df! {
            "Hand" => &starting_hand,
            "2 Players" => &self.player_vecs[0],
            "3 Players" => &self.player_vecs[1],
            "4 Players" => &self.player_vecs[2],
            "5 Players" => &self.player_vecs[3],
            "6 Players" => &self.player_vecs[4],
            "7 Players" => &self.player_vecs[5],
            "8 Players" => &self.player_vecs[6],
            "9 Players" => &self.player_vecs[7],
        }.unwrap();

        std::env::set_var("POLARS_FMT_MAX_ROWS", "100000");
        std::env::set_var("POLARS_FMT_MAX_COLS", "100");

        let sort_options = SortMultipleOptions {
            descending: vec![true, true, true, true, true, true, true, true],             
            limit: None,
            nulls_last: vec![true, true, true, true, true, true, true, true],                    
            maintain_order: true,           
            multithreaded: true,            
        };

        df.sort_in_place(vec!["2 Players", "3 Players", "4 Players", "5 Players", 
        "6 Players", "7 Players", "8 Players", "9 Players"], sort_options);

        let df_str = format!("{:?}", df);

        let mut data_file = File::create("testing_estimates.txt").expect("creation failed");

        data_file.write(df_str.as_bytes()).expect("write failed");
    }
}