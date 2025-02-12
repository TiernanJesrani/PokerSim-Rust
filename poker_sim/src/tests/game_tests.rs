#[cfg(test)]
mod tests {
    use crate::models::game_model;

    #[test]
    fn test_deal() -> () {
        let mut game = game_model::Game::new(5, false, 5, 10);

        game.deal();
        

        assert_eq!(game.deck.cards.len(), game.deck.deck_size - 10);
    }

    #[test]
    fn test_board() -> () {
        let mut game = game_model::Game::new(5, false, 5, 10);

        game.deal();

        game.flop();
        
        assert_eq!(game.board.len(), 3);

        game.turn();

        assert_eq!(game.board.len(), 4);

        game.river();

        assert_eq!(game.board.len(), 5);
    }

    #[test]
    fn test_form_seven() -> () {
        let mut game = game_model::Game::new(5, false, 5, 5);

        game.deal();

        game.flop();

        game.turn();

        game.river();

        assert_eq!(7, game.form_seven_cards(13).seven_cards.len());
    }


    #[test]
    fn test_form_hands() -> () {
        let mut game = game_model::Game::new(5, false, 5, 5);

        game.deal();

        game.flop();

        game.turn();

        game.river();

        game.form_hand_strengths();

        assert_eq!(5, game.hand_strengths.len());
    }
}