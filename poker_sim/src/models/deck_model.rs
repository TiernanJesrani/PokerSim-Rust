use rand::Rng;
use crate::models::card_model::Card;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
    //pub cards_test: [Card; 50]
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
        Deck { cards: card_list, deck_size: 50, }
    }

    pub fn shuffle(&mut self) -> () {
        for i in (1..self.deck_size).rev() {
            let j = rand::rng().random_range(0..=i);
            self.cards.swap(i, j);
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
            if suited == true {
                self.cards.remove(std::cmp::max(rank_1, rank_2));
                self.cards.remove(std::cmp::min(rank_1, rank_2));
            }
            else {
                self.cards.remove(rank_2 + 13);
                self.cards.remove(rank_1);
            }
        }
    }
}
