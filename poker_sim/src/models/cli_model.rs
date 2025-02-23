use crate::models::card_model::Card;
use crate::models::game_model::Game;
use crate::models::monte_model::MonteModel;

pub struct Cli {
    pub hand: Vec<Card>,
    pub board: Vec<Card>,
    pub num_players: usize
}

impl Cli {
    pub fn new(card_1: &str, card_2: &str, num_players: &str, additional_cards: Vec<Option<String>>) -> Cli {
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

        let monte = MonteModel::new(250_000);

        let mut hand = Vec::new();

        hand.push(Card::new(suit_1, rank_1));
        hand.push(Card::new(suit_2, rank_2));

        //let mut board_strings = additional_cards.unwrap_or_else(Vec::new);

        let mut board_strings = Vec::new();

        for i in 0..additional_cards.len() {
            if additional_cards[i].is_some() {
                board_strings.push(additional_cards[i].clone().unwrap());
            }
        }

        let mut board = Vec::new();

        for i in 0..board_strings.len() {
            let suit = board_strings[i].chars().nth(0).expect("empty arg").to_string().parse::<usize>().unwrap();
            let rank = board_strings[i].chars().skip(2).take(2).collect::<String>().parse().unwrap();
            board.push(Card::new(suit, rank))
        }

        Cli { hand: hand.clone(), board: board.clone(), num_players: num_p}
    }
}