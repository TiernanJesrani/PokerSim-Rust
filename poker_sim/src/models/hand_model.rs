use crate::models::card_model::Card;

#[derive(Debug)]
pub struct HandStrength {
    pub seven_cards: Vec<Card>,
    pub hand_type: [u32; 9],
    pub cards_involved: [u32; 14],
    pub cards_leftover: [u32; 14],
    
}

impl HandStrength {
    pub fn new(seven_cards: Vec<Card>) -> HandStrength {
        HandStrength { seven_cards: seven_cards, hand_type: [0; 9], cards_involved: [0; 14], cards_leftover: [0; 14]}
    }

    pub fn convert_if_ace(&self, rank: usize) -> usize {
        if rank == 0 {
            return 13;
        }
        return rank;
    }

    pub fn rank_count(&self) -> [u32; 13] {
        let seven_cards = &self.seven_cards;

        let mut rank_counter: [u32; 13] = [0; 13];

        for i in 0..seven_cards.len() {
            rank_counter[seven_cards[i].rank] += 1;
        }

        rank_counter
    }

    pub fn pairs(&self) -> Vec<usize> {
        let pair_counter = self.rank_count();

        let mut pairs = Vec::new();
        for i in 0..pair_counter.len() {
            if pair_counter[i] > 1 {
                pairs.push(self.convert_if_ace(i));
            }
        }
        pairs.sort();

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
                sets.push(self.convert_if_ace(i));
            }
        }
        sets.sort();

        sets
    }

    // only one possible quad, but this keeps the return types consistent
    pub fn quads(&self) -> Vec<usize> {
        let rank_count = self.rank_count();

        let mut quads = Vec::new();

        for i in 0..rank_count.len() {
            if rank_count[i] > 3 {
                quads.push(self.convert_if_ace(i));
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

        fullhouses.sort_unstable_by_key(|x| (x.0, x.1));

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
                    let x = i - 1;
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
        straights.sort_unstable_by_key(|x| (x.1, x.0));

        straights
    }

    pub fn flushes(&self) -> Vec<usize> {
        let mut flush_counts = vec![
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new()
        ];

        for i in 0..self.seven_cards.len() {
            flush_counts[self.seven_cards[i].suit].push(self.convert_if_ace(self.seven_cards[i].rank));
        }

        for i in 0..flush_counts.len() {
            if flush_counts[i].len() > 4 {
                flush_counts[i].sort();

                return flush_counts[i].clone();
            }
        }

        Vec::new()
    }

    pub fn best_straight(&mut self) -> (usize, bool) {
        let straights = self.straights();

        let mut high_card = 0;
        let mut flush = false;

        if straights.len() > 0 {
            high_card = straights[straights.len() - 1].0;
            flush = straights[straights.len() - 1].1;
        }

        (high_card, flush)
    }

    pub fn best_fullhouse(&self) -> (usize, usize) {
        let fullhouses = self.fullhouses();

        let mut set = 0;
        let mut pair = 0;
        
        if fullhouses.len() > 0 {
            set = fullhouses[fullhouses.len() - 1].0;
            pair = fullhouses[fullhouses.len() - 1].1;
        }

        (set, pair)
    }

    pub fn best_flush(&self) -> Vec<usize> {
        self.flushes()
    }

    pub fn best_quads(&self) -> usize {
        let quads = self.quads();

        if quads.len() > 0 {
            return quads[0];
        }

        0
    }

    pub fn best_set(&self) -> usize {
        let sets = self.sets();

        if sets.len() > 0 {
            return sets[sets.len() - 1];
        }

        0
    }

    pub fn best_two_pair(&self) -> (usize, usize) {
        let two_pairs = self.two_pairs();

        if two_pairs.len() > 0 {
            return two_pairs[two_pairs.len() - 1];
        }

        (0, 0)
    }

    pub fn best_pair(&self) -> usize {
        let pairs = self.pairs();

        if pairs.len() > 0 {
            return pairs[pairs.len() - 1];
        }

        0
    }

    pub fn best_five_combo(&mut self) -> () {

        let straight = self.best_straight();
        if straight.0 != 0 && straight.1 == true {
            self.hand_type[8] = 1;
            self.cards_involved[straight.0] = 1;
            return
        }
        
        let quads = self.best_quads();
        if quads != 0 {
            self.hand_type[7] = 1;
            self.cards_involved[quads] = 1;

            if self.seven_cards.len() > 4 {
                self.add_aces_top();
                let mut i = self.seven_cards.len() - 1;

                while self.seven_cards[i].rank == quads {
                    i -= 1;
                }

                self.cards_leftover[self.seven_cards[i].rank] = 1;
                self.remove_aces_top();
            }
            return
        }

        let fullhouse = self.best_fullhouse();
        if fullhouse.0 != 0 {
            self.hand_type[6] = 1;
            self.cards_involved[fullhouse.0] = 1;
            self.cards_involved[fullhouse.1] = 1;
            self.cards_leftover[fullhouse.1] = 1;
            return
        }

        let flush = self.best_flush();
        if flush.len() != 0 {
            self.hand_type[5] = 1;

            for i in ((flush.len() - 5)..(flush.len())).rev() {
                self.cards_involved[flush[i]] = 1;
            }

            return
        }

        if straight.0 != 0 {
            self.hand_type[4] = 1;
            self.cards_involved[straight.0] = 1;
            return
        }

        let set = self.best_set();
        if set != 0 {
            self.hand_type[3] = 1;
            self.cards_involved[set] = 1;

            self.add_aces_top();
            let mut i = self.seven_cards.len() - 1;

            for _j in 0..2 {
                while i > 0 && self.seven_cards[i].rank == set {
                    i -= 1;
                }
    
                self.cards_leftover[self.seven_cards[i].rank] = 1;
                i -= 1;
            }
            
            self.remove_aces_top();
            return
        }

        let two_pair = self.best_two_pair();
        if two_pair.0 != 0 {
            self.hand_type[2] = 1;
            self.cards_involved[two_pair.0] = 1;
            self.cards_involved[two_pair.1] = 1;

            self.add_aces_top();
            let mut i = self.seven_cards.len() - 1;

            while i > 0 && self.seven_cards[i].rank == set {
                i -= 1;
            }

            self.cards_leftover[self.seven_cards[i].rank] = 1;
            
            self.remove_aces_top();

            return
        }   
        
        let pair = self.best_pair();
        if pair != 0 {
            self.hand_type[1] = 1;
            self.cards_involved[pair] = 1;

            self.add_aces_top();
            let mut i = self.seven_cards.len() - 1;

            for _j in 0..3 {
                while i > 0 && self.seven_cards[i].rank == pair {
                    i -= 1;
                }

                self.cards_leftover[self.seven_cards[i].rank] = 1;

                i -= 1;
            }

            self.remove_aces_top();

            return
        }

        self.hand_type[0] = 1;
        
        self.add_aces_top();

        for i in ((self.seven_cards.len() - 5)..(self.seven_cards.len())).rev() {
            self.cards_involved[self.seven_cards[i].rank] = 1;
        }

        self.remove_aces_top();

        return
    }
}