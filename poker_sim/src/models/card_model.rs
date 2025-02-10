#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Card {
    pub suit: usize, 
    pub rank: usize
}

impl Card  {
    pub fn new(suit: usize, rank: usize) -> Card {
        Card { suit: suit, rank: rank}
    }
}