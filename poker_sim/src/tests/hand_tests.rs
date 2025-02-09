#[cfg(test)]
mod tests {
    use crate::models::game_model;

    #[test]
    fn test_pairs() -> () {
        let mut game = game_model::Game::new(5, false, 5, 5);

        game.deal();

        game.form_hand_strengths();

        assert_eq!(1, game.main_hand_strength.pairs().len());
    }

    #[test]
    fn test_two_pairs() -> () {
        let mut game = game_model::Game::new(5, false, 5, 5);

        game.deal();

        game.form_hand_strengths();

        assert_eq!(0, game.main_hand_strength.two_pairs().len());
    }
}