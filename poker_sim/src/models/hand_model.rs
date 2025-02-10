use crate::models::card_model::Card;

#[derive(Debug)]
pub struct HandStrength {
    pub seven_cards: Vec<Card>
}

impl HandStrength {
    pub fn new(seven_cards: Vec<Card>) -> HandStrength {
        
        HandStrength { seven_cards: seven_cards}
    }

    pub fn rank_count(&self) -> [u32; 13] {
        let seven_cards = &self.seven_cards;

        let mut rank_counter: [u32; 13] = [0; 13];

        for i in 0..seven_cards.len() {
            rank_counter[seven_cards[i].rank] += 1;
        }

        rank_counter
    }

    pub fn suit_count(&self) -> [u32; 4] {
        let seven_cards = &self.seven_cards;

        let mut suit_counter: [u32; 4] = [0; 4];

        for i in 0..seven_cards.len() {
            suit_counter[seven_cards[i].suit] += 1;
        }

        suit_counter
    }

    pub fn pairs(&self) -> Vec<usize> {
        let pair_counter = self.rank_count();

        let mut pairs = Vec::new();
        for i in 0..pair_counter.len() {
            if pair_counter[i] > 1 {
                pairs.push(i);
            }
        }

        pairs
    }

    pub fn two_pairs(&self) -> Vec<(usize, usize)> {
        let pairs = self.pairs();

        let mut two_pairs = Vec::new();
        if pairs.len() > 1 {
            for i in 0..pairs.len() {
                for j in (i + 1)..pairs.len() {
                    two_pairs.push((pairs[i], pairs[j]));
                }
            }
        }

        two_pairs
    }

    pub fn sets(&self) -> Vec<usize> {
        let rank_count = self.rank_count();

        let mut sets = Vec::new();

        for i in 0..rank_count.len() {
            if rank_count[i] > 2 {
                sets.push(i);
            }
        }

        sets
    }

    // only one possible quad, but this keeps the return types consistent
    pub fn quads(&self) -> Vec<usize> {
        let rank_count = self.rank_count();

        let mut quads = Vec::new();

        for i in 0..rank_count.len() {
            if rank_count[i] > 3 {
                quads.push(i);
            }
        }

        quads
    }

    pub fn fullhouses(&self) -> Vec<(usize, usize)> {
        let mut fullhouses = Vec::new();

        let pairs = self.pairs();

        let sets = self.sets();

        for i in 0..sets.len() {
            for j in 0..pairs.len() {
                if sets[i] != pairs[j] {
                    fullhouses.push((sets[i], pairs[j]));
                }
            }
        }

        fullhouses
    }
    pub fn add_aces_top(&mut self) -> () {
        let mut i = 0;
        while self.seven_cards[i].rank == 0 {
            self.seven_cards.push(Card::new(self.seven_cards[i].suit, 13));
            i += 1;
        }
    }

    pub fn remove_aces_top(&mut self) -> () {
        while self.seven_cards[self.seven_cards.len() - 1].rank == 13 {
            self.seven_cards.pop();
        }
    }

    pub fn straights(&mut self) -> Vec<(usize, bool)> {
        self.add_aces_top();
        let mut straights = Vec::new();
        println!("{:?}", self.seven_cards);
        for x in 1..self.seven_cards.len() {
            if self.seven_cards[x].rank == self.seven_cards[x - 1].rank {
                continue;
            }
            let mut counter = 1;
            let mut suited = true;
            for i in x..self.seven_cards.len() {
                if self.seven_cards[i].rank == self.seven_cards[i - 1].rank + 1 {
                    counter += 1;
                    if self.seven_cards[i].suit != self.seven_cards[i - 1].suit {
                        suited = false;
    
                        if i > 1 && self.seven_cards[i - 2].rank == self.seven_cards[i - 1].rank && 
                        self.seven_cards[i - 2].suit == self.seven_cards[i].suit {
                            suited = true; 
                        }
                    }
                }
                else if self.seven_cards[i].rank == self.seven_cards[i - 1].rank {
                    if !suited {
                        let mut y = i;
                        while y > 1 && self.seven_cards[y].rank == self.seven_cards[y - 1].rank {
                            y -= 1;
                        }
                        if self.seven_cards[y - 1].suit == self.seven_cards[i].suit {
                            suited = true;
                        }
                    }
                }
                else {
                    break;
                }
                if counter == 5 {
                    let mut x = i - 1;
                    let mut y = i + 1;
                    if !suited {
                        while y < self.seven_cards.len() && self.seven_cards[y].rank == self.seven_cards[i].rank {
                            if self.seven_cards[y].suit == self.seven_cards[x].suit {
                                suited = true;
                            }
                            y += 1;
                        }
                    }
                    straights.push((self.seven_cards[i].rank, suited));
                    break;
                }
            }
        }
        self.remove_aces_top();

        straights
    }
}