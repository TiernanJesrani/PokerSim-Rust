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

        let main_hand = if suited {
            vec![deck.cards[rank_1], deck.cards[rank_2]]
        } else {
            vec![deck.cards[rank_1], deck.cards[rank_2 + 13]]
        };
    
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
        let hand_vec = if hand >= self.players.len() {
            self.main_hand.to_vec()
        } else {
            self.players[hand].hand.to_vec()
        };
        let mut board_and_hand = [self.board.to_vec(), hand_vec].concat();
        board_and_hand.sort_by_key(|x| x.rank);

        HandStrength::new(board_and_hand)
    }

    pub fn form_hand_strengths(&mut self) -> () {
        self.main_hand_strength = self.form_seven_cards(self.players.len());
        self.main_hand_strength.best_five_combo();

        for i in 0..self.players.len() {
            self.hand_strengths.push(self.form_seven_cards(i));
            //self.hand_strengths[i].best_five_combo();
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