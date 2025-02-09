#[cfg(test)]
mod tests {
    use crate::models::deck_model;

    #[test]
    fn test_shuffle() -> () {
        let deck = deck_model::Deck::new();

        let mut deck_shuffle = deck_model::Deck::new();

        deck_shuffle.shuffle();

        assert_ne!(deck.cards[..], deck_shuffle.cards[..]);
    }

    #[test]
    fn test_top() -> () {
        let mut deck = deck_model::Deck::new();

        deck.top_card();

        assert_eq!(deck.cards.len(), deck.deck_size + 1);

        deck.top_card();

        assert_eq!(deck.cards.len(), deck.deck_size);
    }

    #[test]
    fn test_remove() -> () {
        let mut deck = deck_model::Deck::new();

        deck.remove_cards(false, 0, 10);

        assert_eq!(deck.cards.len(), deck.deck_size);

        assert_ne!(deck.cards[0].rank, 0);
    }

    #[test]
    #[should_panic]
    fn test_remove_suited_pairs() ->() {
        let mut deck = deck_model::Deck::new();

        deck.remove_cards(true, 5, 5);
    }

    #[test]
    #[should_panic]
    fn test_remove_high_rank() ->() {
        let mut deck = deck_model::Deck::new();

        deck.remove_cards(true, 15, 0);
    }
}