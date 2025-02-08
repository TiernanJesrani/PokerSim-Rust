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

// impl Game {
//     pub fn new(u32 num_players, bool suited, u32 rank_1, u32 rank_2) -> Game {
//         deck = Deck::new();
        


//         // before shuffling we remove the chosen cards from the deck.
//         deck.shuffle();

        
        
//     }

//     pub fn deal(self) {
//         for i in 0..(self.players.len() * 2) {

//         }
//     }
// }