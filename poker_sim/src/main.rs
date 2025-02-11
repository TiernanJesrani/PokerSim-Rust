mod models;
mod tests;

fn main() {
    let mut game = models::game_model::Game::new(5, false, 1, 1);

    game.deal();

    let card = models::card_model::Card::new(2, 1);

    game.board.push(card);

    let card = models::card_model::Card::new(0, 10);

    game.board.push(card);

    let card = models::card_model::Card::new(0, 0);

    game.board.push(card);

    let card = models::card_model::Card::new(2, 11);

    game.board.push(card);

    game.form_hand_strengths();

    game.main_hand_strength.best_five_combo();

    //assert_eq!(game.main_hand_strength.cards_leftover[13], 1);

    println!("{:?}", game.main_hand_strength.cards_leftover);
    
}
