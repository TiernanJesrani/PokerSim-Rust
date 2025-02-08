// Here we define our various models needed for the poker simulator

#[derive(Debug)]
pub struct Deck {
    pub cards: [Card; 52]
}

#[derive(Copy, Debug, Clone)]
pub struct Card {
    pub suit: u64, 
    pub value: u64
}