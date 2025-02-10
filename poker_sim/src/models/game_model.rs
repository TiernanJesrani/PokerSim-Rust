use crate::models::player_model::Player;
use crate::models::deck_model::Deck;
use crate::models::card_model::Card;
use crate::models::hand_model::HandStrength;

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub deck: Deck,
    pub board: Vec<Card>,
    pub main_hand: Vec<Card>,
    pub hand_strengths: Vec<HandStrength>,
    pub main_hand_strength: HandStrength
}

impl Game {
    pub fn new(num_players_minus_one: usize, suited: bool, rank_1: usize, rank_2: usize) -> Game {
        let mut deck = Deck::new();

        let mut main_hand = Vec::new();

        main_hand.push(deck.cards[rank_1]);

        if suited == true {
            main_hand.push(deck.cards[rank_2]);
        }
        else {
            main_hand.push(deck.cards[rank_2 + 13]); // offsetting by the number of ranks to move to the next suit
        }

        deck.remove_cards(suited, rank_1, rank_2);

        deck.shuffle();
        
        Game { players: vec![Player { hand: Vec::new() }; num_players_minus_one], 
        deck: deck, board: Vec::new(), main_hand: main_hand, main_hand_strength: 
        HandStrength::new(Vec::new()), hand_strengths: Vec::new()}
    }

    pub fn deal(&mut self) {
        for i in 0..(self.players.len() * 2) {
            let player_num = i % self.players.len();
            self.players[player_num].hand.push(self.deck.top_card());
        }
    }

    pub fn flop(&mut self) {
        for _i in 0..3 {
            self.board.push(self.deck.top_card());
        }
    }

    pub fn turn(&mut self) {
        self.board.push(self.deck.top_card());
    }

    pub fn river(&mut self) {
        self.board.push(self.deck.top_card());
    }

    // This section deals with checking for the strength of the hands.

    pub fn form_seven_cards(&self, hand: usize) -> HandStrength {
        let mut hand_vec = Vec::new();
        if hand >= self.players.len() {
            hand_vec = self.main_hand.to_vec();
        }
        else {
            hand_vec = self.players[hand].hand.to_vec();
        }
        let mut board_and_hand = [self.board.to_vec(), hand_vec].concat();
        board_and_hand.sort_by_key(|x| x.rank);

        HandStrength::new(board_and_hand)
    }

    pub fn form_hand_strengths(&mut self) -> () {
        self.main_hand_strength = self.form_seven_cards(self.players.len());

        for i in 0..self.players.len() {
            self.hand_strengths.push(self.form_seven_cards(i));
        }
    }
}