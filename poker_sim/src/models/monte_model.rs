use crate::models::game_model::Game;
use polars::df;

pub struct MonteModel {
    estimates: usize,
    conversion_string: [&'static str; 12],
    player_vecs: Vec<Vec<f32>>
}

impl MonteModel {
    pub fn new(estimates: usize) -> MonteModel {
        let mut player_vecs = vec![
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

        MonteModel { estimates: estimates, conversion_string: ["A", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K"], player_vecs: player_vecs}
    }

    fn run_one_hand(&mut self, suited: bool, rank_1: usize, rank_2: usize, num_players: usize) -> f32 {
        let mut sum = 0;

        for i in 0..self.estimates {
            let mut game = Game::new(num_players - 1, suited, rank_1, rank_2);

            game.deal();

            game.flop();

            game.turn();

            game.river();

            game.form_hand_strengths();

            sum += game.main_wins();
        }

        sum as f32 / (self.estimates / 100) as f32
    }

    pub fn run_sim(&mut self) -> () {
        let mut starting_hand = Vec::new();

        for i in 0..1 {
            for j in i..1 {
                let hand_name_o = self.conversion_string[i].to_owned() + self.conversion_string[j] + "o";
                starting_hand.push(hand_name_o);
                for k in 2..4 {
                    let hand_result = self.run_one_hand(false, i, j, k);
                    self.player_vecs[k - 2].push(hand_result);
                }
                
                if i != j {
                    let hand_name_s = self.conversion_string[i].to_owned() + self.conversion_string[j] + "s";
                    starting_hand.push(hand_name_s);
                    for k in 2..4 {
                        let hand_result = self.run_one_hand(true, i, j, k);
                        self.player_vecs[k - 2].push(hand_result);
                    }
                }
            }
        }

        let df = df! {
            "Hand" => &starting_hand,
            "2 Players" => &self.player_vecs[0],
            "3 Players" => &self.player_vecs[1],
            // "4 Players" => &self.player_vecs[2],
            // "5 Players" => &self.player_vecs[3],
            // "6 Players" => &self.player_vecs[4],
            // "7 Players" => &self.player_vecs[5],
            // "8 Players" => &self.player_vecs[6],
            // "9 Players" => &self.player_vecs[7],
        }.unwrap();

        println!("{:?}", df);
    }
}