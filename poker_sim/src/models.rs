use rand::Rng;
// Here we define our various models needed for the poker simulator

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
        Deck { cards: card_list, deck_size: 52}
    }

    pub fn shuffle(&mut self) -> () {
        for _i in 0..(self.deck_size * 2) {
            let pos_one = rand::rng().random_range(0..self.deck_size);
            let pos_two = rand::rng().random_range(0..self.deck_size);
            self.cards.swap(pos_one, pos_two)
        }
    }
}

#[derive(Copy, Debug, Clone)]
pub struct Card {
    pub suit: u32, 
    pub rank: u32
}

#[derive(Copy, Debug, Clone)]
pub struct Player {
    pub hand: [Card; 2]
}

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub deck: Deck
}