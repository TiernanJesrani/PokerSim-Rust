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
    pub main_hand_strength: HandStrength,
    pub cli_board: Vec<Card>,
}

impl Game {
    pub fn new(num_players_minus_one: usize, card_1: Card, card_2: Card, additional_cards: Option<Vec<Card>>) -> Game {
        let mut deck = Deck::new();

        let main_hand = vec![card_1, card_2];

        let mut board = additional_cards.unwrap_or_else(Vec::new);

        board.push(card_1);
        board.push(card_2);
        
        deck.remove_cards(board.clone());

        board.pop();
        board.pop();

        deck.shuffle();

        Game { players: vec![Player { hand: Vec::new() }; num_players_minus_one], 
        deck: deck, board: board.clone(), main_hand: main_hand, main_hand_strength: 
        HandStrength::new(Vec::new()), hand_strengths: Vec::new(), cli_board: board.clone()}
    }

    pub fn must_shuffle(&mut self) -> () {
        if self.deck.deck_size - self.deck.deck_pos < (self.players.len() * 2) + 7 {
            self.deck.deck_pos = 0;
            self.deck.shuffle();
        }
    }

    pub fn reset_game(&mut self) -> () {
        self.must_shuffle();
        self.players.iter_mut().for_each(|v| v.hand.clear());
        self.board = self.cli_board.clone();
        self.hand_strengths.clear();
    }

    pub fn deal(&mut self) -> () {
        for i in 0..(self.players.len() * 2) {
            let player_num = i % self.players.len();
            self.players[player_num].hand.push(self.deck.top_card());
        }
    }

    pub fn flop(&mut self) -> () {
        for _i in 0..3 {
            self.board.push(self.deck.top_card());
        }
    }

    pub fn turn(&mut self) -> () {
        self.board.push(self.deck.top_card());
    }

    pub fn river(&mut self) {
        self.board.push(self.deck.top_card());
    }

    pub fn form_seven_cards(&self, hand: usize) -> HandStrength {
        let hand_slice = if hand >= self.players.len() {
            &self.main_hand
        } else {
            &self.players[hand].hand
        };
    
        let mut board_and_hand = Vec::with_capacity(self.board.len() + hand_slice.len());
        board_and_hand.extend_from_slice(&self.board);
        board_and_hand.extend_from_slice(hand_slice);
        
        board_and_hand.sort_by_key(|x| x.rank);
    
        HandStrength::new(board_and_hand)
    }

    pub fn form_hand_strengths(&mut self) -> () {
        self.main_hand_strength = self.form_seven_cards(self.players.len());
        self.main_hand_strength.best_five_combo();

        for i in 0..self.players.len() {
            self.hand_strengths.push(self.form_seven_cards(i));
        }
    }

    pub fn beats_main_hand(&self, player_pos: usize) -> u32 {
        for i in 0..self.main_hand_strength.hand_type.len() {
            if self.main_hand_strength.hand_type[i] != self.hand_strengths[player_pos].hand_type[i] {
                return self.main_hand_strength.hand_type[i];
            }
            else if self.main_hand_strength.hand_type[i] == 1 {
                for j in (0..self.main_hand_strength.cards_involved.len()).rev() {
                    if self.main_hand_strength.cards_involved[j] != self.hand_strengths[player_pos].cards_involved[j] {
                        return self.hand_strengths[player_pos].cards_involved[j];
                    }
                }
                for k in (0..self.main_hand_strength.cards_leftover.len()).rev() {
                    if self.main_hand_strength.cards_leftover[k] != self.hand_strengths[player_pos].cards_leftover[k] {
                        return self.hand_strengths[player_pos].cards_leftover[k];
                    }
                }
            }
        }
        1
    }

    pub fn main_wins(&mut self) -> u32 {
        for i in 0..self.hand_strengths.len() {
            self.hand_strengths[i].best_five_combo();
            if self.beats_main_hand(i) == 1 {
                return 0
            }
        }
        1
    }
}