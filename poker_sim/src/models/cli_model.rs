use crate::models::card_model::Card;
use crate::models::game_model::Game;
use crate::models::monte_model::MonteModel;

pub struct Cli {
    pub hand: Vec<Card>,
    pub board: Vec<Card>,
    pub num_players: usize
}

impl Cli {
    pub fn new(card_1: &str, card_2: &str, num_players: &str) -> Cli {
        let suit_1 = card_1.chars().nth(0).expect("empty arg").to_string().parse::<usize>().unwrap();
        let rank_1 = card_1.chars().skip(2).take(2).collect::<String>().parse().unwrap();
        
        let suit_2 = card_2.chars().nth(0).expect("empty arg").to_string().parse::<usize>().unwrap();
        let rank_2 = card_2.chars().skip(2).take(2).collect::<String>().parse().unwrap();

        let num_p = num_players.parse::<usize>().unwrap();

        let suited = if suit_2 == suit_1 {
            true
        } else {
            false
        };

        if rank_1 > 12 {
            println!("ERROR: Rank 1 greater than allowed!");
            panic!();
        }
        if rank_2 > 12 {
            println!("ERROR: Rank 2 greater than allowed!");
            panic!();
        }
        if rank_1 == rank_2 && suited == true {
            println!("ERROR: Pairs cannot be suited!");
            panic!();
        }
        if num_p < 2 || num_p > 9 {
            println!("ERROR: Number of players less than or greater than allowed!");
            panic!();
        }

        let mut game = Game::new(num_p, suited, rank_1, rank_2);

        let mut v = Vec::new();

        v.push(Card::new(suit_1, rank_1));
        v.push(Card::new(suit_2, rank_2));

        println!("card 1: {:?}, card 2: {:?}", suit_1, rank_1);
        println!("card 1: {:?}, card 2: {:?}", suit_2, rank_2);

        Cli { hand: v.clone(), board: v.clone(), num_players: 1}
    }
}