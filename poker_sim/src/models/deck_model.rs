use rand::Rng;
use crate::models::card_model::Card;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub deck_size: usize
}

impl Deck {
    pub fn new() -> Deck {
        let mut card_list = Vec::new();
        for i in 0..4 {
            for j in 0..13 {
                card_list.push(Card { suit: i, rank: j});
            }
        }
        Deck { cards: card_list, deck_size: 50}
    }

    pub fn shuffle(&mut self) -> () {
        for _i in 0..((self.deck_size) * 40) {
            let pos_one = rand::rng().random_range(0..self.deck_size);
            let pos_two = rand::rng().random_range(0..self.deck_size);
            self.cards.swap(pos_one, pos_two)
        }
    }

    pub fn top_card(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

    pub fn remove_cards(&mut self, suited: bool, rank_1: usize, rank_2: usize) -> () {
        // ALL OF THIS ERROR CHECKING NEEDS TO BE MOVED TO THE CLI LATER. DO NOT LEAVE HERE.
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
        else {
            self.cards.remove(rank_1);

            if suited == true {
                self.cards.remove(rank_2);
            }
            else {
                self.cards.remove(rank_2 + 13); // offsetting by the number of ranks to move to the next suit
            }
        }
    }
}
