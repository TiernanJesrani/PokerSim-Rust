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
}