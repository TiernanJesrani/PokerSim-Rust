use rand::Rng;
use crate::models::card_model::Card;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub deck_size: usize,
    pub deck_pos: usize,
}

impl Deck {
    pub fn new() -> Deck {
        let mut card_list = Vec::new();
        for i in 0..4 {
            for j in 0..13 {
                card_list.push(Card { suit: i, rank: j});
            }
        }
        Deck { cards: card_list, deck_size: 50, deck_pos: 0}
    }

    pub fn shuffle(&mut self) -> () {
        for i in (1..self.deck_size).rev() {
            let j = rand::rng().random_range(0..=i);
            self.cards.swap(i, j);
        }
    }

    pub fn top_card(&mut self) -> Card {
        self.deck_pos += 1;
        self.cards[self.deck_pos - 1]
    }

    pub fn remove_cards(&mut self, mut cards: Vec<Card>) -> () {
        cards.sort_by(|a, b| {
            b.suit.cmp(&a.suit)
                .then_with(|| b.rank.cmp(&a.rank))
        });
        for card in &cards {
            if let Some(pos) = self.cards.iter().position(|c| c == card) {
                self.cards.remove(pos); 
            }
        }
        self.deck_size -= cards.len();
    }
}
