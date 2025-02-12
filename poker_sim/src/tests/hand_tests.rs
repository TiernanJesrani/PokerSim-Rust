#[cfg(test)]
mod tests {
    use crate::models::game_model;
    use crate::models::card_model;

    #[test]
    fn test_pairs() -> () {
        let mut game = game_model::Game::new(5, false, 5, 5);

        game.deal();

        let card = card_model::Card::new(3, 1);

        game.board.push(card);

        let card = card_model::Card::new(3, 2);

        game.board.push(card);

        let card = card_model::Card::new(3, 7);

        game.board.push(card);

        let card = card_model::Card::new(3, 8);

        game.board.push(card);

        let card = card_model::Card::new(3, 10);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!(1, game.main_hand_strength.pairs().len());
    }

    #[test]
    fn test_two_pairs() -> () {
        let mut game = game_model::Game::new(5, false, 5, 5);

        game.deal();

        let card = card_model::Card::new(3, 6);

        game.board.push(card);

        let card = card_model::Card::new(3, 6);

        game.board.push(card);

        let card = card_model::Card::new(3, 7);

        game.board.push(card);

        let card = card_model::Card::new(3, 8);

        game.board.push(card);

        let card = card_model::Card::new(3, 11);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!(1, game.main_hand_strength.two_pairs().len());
    }

    #[test]
    fn test_sets() -> () {
        let mut game = game_model::Game::new(5, false, 5, 5);

        game.deal();

        let card = card_model::Card::new(3, 5);

        game.board.push(card);

        let card = card_model::Card::new(3, 4);

        game.board.push(card);

        let card = card_model::Card::new(0, 4);

        game.board.push(card);

        let card = card_model::Card::new(1, 4);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!(4, game.main_hand_strength.sets()[0]);
    }

    #[test]
    fn test_quads() -> () {
        let mut game = game_model::Game::new(5, false, 5, 5);

        game.deal();

        let card = card_model::Card::new(3, 5);

        game.board.push(card);

        let card = card_model::Card::new(1, 5);

        game.board.push(card);

        let card = card_model::Card::new(3, 5);

        game.board.push(card);

        let card = card_model::Card::new(1, 10);

        game.board.push(card);

        let card = card_model::Card::new(1, 7);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!(1, game.main_hand_strength.sets().len());
    }

    #[test]
    fn test_fullhouses() -> () {
        let mut game = game_model::Game::new(5, false, 5, 5);

        game.deal();

        let card = card_model::Card::new(3, 6);

        game.board.push(card);

        let card = card_model::Card::new(2, 6);

        game.board.push(card);

        let card = card_model::Card::new(1, 6);

        game.board.push(card);

        let card = card_model::Card::new(3, 5);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!(2, game.main_hand_strength.fullhouses().len());
    }

    #[test]
    fn test_straight_normal() -> () {
        let mut game = game_model::Game::new(5, true, 5, 6);

        game.deal();

        let card = card_model::Card::new(0, 7);

        game.board.push(card);

        let card = card_model::Card::new(0, 8);

        game.board.push(card);

        let card = card_model::Card::new(1, 9);

        game.board.push(card);

        let card = card_model::Card::new(2, 10);

        game.board.push(card);

        let card = card_model::Card::new(0, 11);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!((9, false), game.main_hand_strength.straights()[0]);
    }

    #[test]
    fn test_straight_flush() -> () {
        let mut game = game_model::Game::new(5, true, 5, 6);

        game.deal();

        let card = card_model::Card::new(0, 7);

        game.board.push(card);

        let card = card_model::Card::new(0, 8);

        game.board.push(card);

        let card = card_model::Card::new(0, 9);

        game.board.push(card);

        let card = card_model::Card::new(2, 10);

        game.board.push(card);

        let card = card_model::Card::new(0, 11);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!((10, false), game.main_hand_strength.straights()[0]);
    }

    #[test]
    fn test_straight_ace_low() -> () {
        let mut game = game_model::Game::new(5, true, 2, 1);

        game.deal();

        let card = card_model::Card::new(0, 0);

        game.board.push(card);

        let card = card_model::Card::new(1, 0);

        game.board.push(card);

        let card = card_model::Card::new(0, 4);

        game.board.push(card);

        let card = card_model::Card::new(0, 3);

        game.board.push(card);

        let card = card_model::Card::new(0, 11);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!((4, true), game.main_hand_strength.straights()[0]);
    }

    #[test]
    fn test_ace_high() -> () {
        let mut game = game_model::Game::new(5, true, 10, 11);

        game.deal();

        let card = card_model::Card::new(0, 0);

        game.board.push(card);

        let card = card_model::Card::new(0, 12);

        game.board.push(card);

        let card = card_model::Card::new(0, 9);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!((13, true), game.main_hand_strength.straights()[0]);
    }

    #[test]
    fn test_flushes() -> () {
        let mut game = game_model::Game::new(5, true, 10, 11);

        game.deal();

        let card = card_model::Card::new(0, 3);

        game.board.push(card);

        let card = card_model::Card::new(0, 12);

        game.board.push(card);

        let card = card_model::Card::new(0, 9);

        game.board.push(card);

        let card = card_model::Card::new(1, 10);

        game.board.push(card);

        let card = card_model::Card::new(2, 8);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!(3, game.main_hand_strength.flushes()[0]);
    }

    #[test]
    fn test_flushes_ace() -> () {
        let mut game = game_model::Game::new(5, true, 10, 11);

        game.deal();

        let card = card_model::Card::new(0, 3);

        game.board.push(card);

        let card = card_model::Card::new(0, 0);

        game.board.push(card);

        let card = card_model::Card::new(0, 9);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!(3, game.main_hand_strength.flushes()[0]);
    }

    #[test]
    fn test_best_straight() -> () {
        let mut game = game_model::Game::new(5, true, 1, 2);

        game.deal();

        let card = card_model::Card::new(0, 3);

        game.board.push(card);

        let card = card_model::Card::new(0, 0);

        game.board.push(card);

        let card = card_model::Card::new(0, 4);

        game.board.push(card);

        let card = card_model::Card::new(1, 5);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!((4, true), game.main_hand_strength.best_straight());

        let mut game = game_model::Game::new(5, false, 1, 2);

        game.deal();

        let card = card_model::Card::new(0, 3);

        game.board.push(card);

        let card = card_model::Card::new(0, 0);

        game.board.push(card);

        let card = card_model::Card::new(0, 4);

        game.board.push(card);

        let card = card_model::Card::new(1, 5);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!((5, false), game.main_hand_strength.best_straight());
    }

    #[test] 
    fn test_best_fullhouse() -> () {
        let mut game = game_model::Game::new(5, false, 1, 1);

        game.deal();

        let card = card_model::Card::new(3, 1);

        game.board.push(card);

        let card = card_model::Card::new(0, 0);

        game.board.push(card);

        let card = card_model::Card::new(3, 0);

        game.board.push(card);

        let card = card_model::Card::new(1, 0);

        game.board.push(card);

        game.form_hand_strengths();

        assert_eq!((13, 1), game.main_hand_strength.best_fullhouse());
    }

    #[test]
    fn test_pair_grid() -> () { 
        let mut game = game_model::Game::new(5, false, 1, 1);

        game.deal();

        let card = card_model::Card::new(3, 0);

        game.board.push(card);

        let card = card_model::Card::new(2, 4);

        game.board.push(card);

        let card = card_model::Card::new(3, 3);

        game.board.push(card);

        let card = card_model::Card::new(3, 8);

        game.board.push(card);

        game.form_hand_strengths();

        //game.main_hand_strength.best_five_combo();

        assert_eq!(game.main_hand_strength.hand_type[1], 1);
        assert_eq!(game.main_hand_strength.cards_involved[1], 1);
        assert_eq!(game.main_hand_strength.cards_leftover[13], 1);
        assert_eq!(game.main_hand_strength.cards_leftover[4], 1);
        assert_eq!(game.main_hand_strength.cards_leftover[8], 1);
    }

    #[test]
    fn test_two_pair_grid() -> () { 
        let mut game = game_model::Game::new(5, false, 2, 1);

        game.deal();

        let card = card_model::Card::new(3, 2);

        game.board.push(card);

        let card = card_model::Card::new(2, 1);

        game.board.push(card);

        let card = card_model::Card::new(3, 3);

        game.board.push(card);

        let card = card_model::Card::new(3, 8);

        game.board.push(card);

        game.form_hand_strengths();

        //game.main_hand_strength.best_five_combo();

        assert_eq!(game.main_hand_strength.hand_type[2], 1);
        assert_eq!(game.main_hand_strength.cards_involved[1], 1);
        assert_eq!(game.main_hand_strength.cards_involved[2], 1);
        assert_eq!(game.main_hand_strength.cards_leftover[8], 1);
    }

    #[test]
    fn test_set_grid() -> () { 
        let mut game = game_model::Game::new(5, false, 1, 1);

        game.deal();

        let card = card_model::Card::new(3, 1);

        game.board.push(card);

        let card = card_model::Card::new(2, 4);

        game.board.push(card);

        let card = card_model::Card::new(3, 3);

        game.board.push(card);

        let card = card_model::Card::new(3, 8);

        game.board.push(card);

        game.form_hand_strengths();

        //game.main_hand_strength.best_five_combo();

        assert_eq!(game.main_hand_strength.hand_type[3], 1);
        assert_eq!(game.main_hand_strength.cards_involved[1], 1);
        assert_eq!(game.main_hand_strength.cards_leftover[4], 1);
        assert_eq!(game.main_hand_strength.cards_leftover[8], 1);
    }

    #[test]
    fn test_straight_grid() -> () { 
        let mut game = game_model::Game::new(5, false, 1, 2);

        game.deal();

        let card = card_model::Card::new(3, 3);

        game.board.push(card);

        let card = card_model::Card::new(2, 4);

        game.board.push(card);

        let card = card_model::Card::new(3, 0);

        game.board.push(card);

        let card = card_model::Card::new(3, 8);

        game.board.push(card);

        game.form_hand_strengths();

        //game.main_hand_strength.best_five_combo();

        assert_eq!(game.main_hand_strength.hand_type[4], 1);
        assert_eq!(game.main_hand_strength.cards_involved[4], 1);
    }

    #[test]
    fn test_flush_grid() -> () { 
        let mut game = game_model::Game::new(5, true, 0, 1);

        game.deal();

        let card = card_model::Card::new(0, 2);

        game.board.push(card);

        let card = card_model::Card::new(0, 7);

        game.board.push(card);

        let card = card_model::Card::new(0, 11);

        game.board.push(card);

        let card = card_model::Card::new(0, 12);

        game.board.push(card);

        game.form_hand_strengths();

        //game.main_hand_strength.best_five_combo();

        assert_eq!(game.main_hand_strength.hand_type[5], 1);
        assert_eq!(game.main_hand_strength.cards_involved[13], 1);
        assert_eq!(game.main_hand_strength.cards_involved[12], 1);
        assert_eq!(game.main_hand_strength.cards_involved[11], 1);
        assert_eq!(game.main_hand_strength.cards_involved[7], 1);
        assert_eq!(game.main_hand_strength.cards_involved[2], 1);
    }

    #[test]
    fn test_fullhouse_grid() -> () { 
        let mut game = game_model::Game::new(5, true, 0, 1);

        game.deal();

        let card = card_model::Card::new(1, 1);

        game.board.push(card);

        let card = card_model::Card::new(1, 0);

        game.board.push(card);

        let card = card_model::Card::new(3, 0);

        game.board.push(card);

        let card = card_model::Card::new(2, 12);

        game.board.push(card);

        let card = card_model::Card::new(2, 5);

        game.board.push(card);

        game.form_hand_strengths();

        //game.main_hand_strength.best_five_combo();

        assert_eq!(game.main_hand_strength.hand_type[6], 1);
        assert_eq!(game.main_hand_strength.cards_involved[13], 1);
        assert_eq!(game.main_hand_strength.cards_leftover[1], 1);
    }

    #[test]
    fn test_quads_grid() -> () {
        let mut game = game_model::Game::new(5, false, 1, 1);

        game.deal();

        let card = card_model::Card::new(3, 1);

        game.board.push(card);

        let card = card_model::Card::new(2, 1);

        game.board.push(card);

        let card = card_model::Card::new(3, 0);

        game.board.push(card);

        let card = card_model::Card::new(3, 8);

        game.board.push(card);

        game.form_hand_strengths();

        //game.main_hand_strength.best_five_combo();

        assert_eq!(game.main_hand_strength.hand_type[7], 1);
        assert_eq!(game.main_hand_strength.cards_involved[1], 1);
        assert_eq!(game.main_hand_strength.cards_leftover[13], 1);
    }

    #[test]
    fn test_straight_flush_grid() -> () { 
        let mut game = game_model::Game::new(5, true, 0, 2);

        game.deal();

        let card = card_model::Card::new(0, 3);

        game.board.push(card);

        let card = card_model::Card::new(0, 4);

        game.board.push(card);

        let card = card_model::Card::new(0, 5);

        game.board.push(card);

        let card = card_model::Card::new(0, 6);

        game.board.push(card);

        game.form_hand_strengths();

        //game.main_hand_strength.best_five_combo();

        assert_eq!(game.main_hand_strength.hand_type[8], 1);
        assert_eq!(game.main_hand_strength.cards_involved[6], 1);
    }
}