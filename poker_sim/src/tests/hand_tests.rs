#[cfg(test)]
mod tests {
    use crate::models::game_model;
    use crate::models::card_model;

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

        let mut card = card_model::Card::new(3, 6);

        game.board.push(card);

        let mut card = card_model::Card::new(4, 6);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!(1, game.main_hand_strength.two_pairs().len());
    }

    #[test]
    fn test_sets() -> () {
        let mut game = game_model::Game::new(5, false, 5, 5);

        game.deal();

        let mut card = card_model::Card::new(3, 5);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!(1, game.main_hand_strength.sets().len());
    }

    #[test]
    fn test_quads() -> () {
        let mut game = game_model::Game::new(5, false, 5, 5);

        game.deal();

        let mut card = card_model::Card::new(3, 5);

        game.board.push(card);

        let mut card = card_model::Card::new(4, 5);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!(1, game.main_hand_strength.sets().len());
    }

    #[test]
    fn test_fullhouses() -> () {
        let mut game = game_model::Game::new(5, false, 5, 5);

        game.deal();

        let mut card = card_model::Card::new(3, 6);

        game.board.push(card);

        let mut card = card_model::Card::new(4, 6);

        game.board.push(card);

        let mut card = card_model::Card::new(1, 6);

        game.board.push(card);

        let mut card = card_model::Card::new(3, 5);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!(2, game.main_hand_strength.fullhouses().len());
    }

    #[test]
    fn test_straight_normal() -> () {
        let mut game = game_model::Game::new(5, true, 5, 6);

        game.deal();

        let mut card = card_model::Card::new(0, 7);

        game.board.push(card);

        let mut card = card_model::Card::new(0, 8);

        game.board.push(card);

        let mut card = card_model::Card::new(1, 9);

        game.board.push(card);

        let mut card = card_model::Card::new(2, 10);

        game.board.push(card);

        let mut card = card_model::Card::new(0, 11);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!((9, false), game.main_hand_strength.straights()[0]);
    }

    #[test]
    fn test_straight_flush() -> () {
        let mut game = game_model::Game::new(5, true, 5, 6);

        game.deal();

        let mut card = card_model::Card::new(0, 7);

        game.board.push(card);

        let mut card = card_model::Card::new(0, 8);

        game.board.push(card);

        let mut card = card_model::Card::new(0, 9);

        game.board.push(card);

        let mut card = card_model::Card::new(2, 10);

        game.board.push(card);

        let mut card = card_model::Card::new(0, 11);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!((9, true), game.main_hand_strength.straights()[0]);
    }

    #[test]
    fn test_straight_ace_low() -> () {
        let mut game = game_model::Game::new(5, true, 2, 1);

        game.deal();

        let mut card = card_model::Card::new(0, 0);

        game.board.push(card);

        let mut card = card_model::Card::new(1, 0);

        game.board.push(card);

        let mut card = card_model::Card::new(0, 4);

        game.board.push(card);

        let mut card = card_model::Card::new(0, 3);

        game.board.push(card);

        let mut card = card_model::Card::new(0, 11);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!((4, true), game.main_hand_strength.straights()[0]);
    }

    #[test]
    fn test_ace_high() -> () {
        let mut game = game_model::Game::new(5, true, 10, 11);

        game.deal();

        let mut card = card_model::Card::new(0, 0);

        game.board.push(card);

        let mut card = card_model::Card::new(0, 12);

        game.board.push(card);

        let mut card = card_model::Card::new(0, 9);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!((13, true), game.main_hand_strength.straights()[0]);
    }
}