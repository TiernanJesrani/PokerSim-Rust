use rand::Rng;
// Here we define our various models needed for the poker simulator
#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Card {
    pub suit: u32, 
    pub rank: u32
}

#[derive(Debug, Clone)]
pub struct Player {
    pub hand: Vec<Card>
}

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub deck: Deck
}