mod models;
mod tests;

fn main() {
    let mut game = models::game_model::Game::new(5, true, 11, 12);

    game.deal();

    let mut card = models::card_model::Card::new(0, 10);

    game.board.push(card);

    let mut card = models::card_model::Card::new(0, 9);

    game.board.push(card);

    let mut card = models::card_model::Card::new(1, 9);

    game.board.push(card);

    let mut card = models::card_model::Card::new(2, 0);

    game.board.push(card);

    let mut card = models::card_model::Card::new(0, 0);

    game.board.push(card);

    game.form_hand_strengths();

    println!("{:?}", game.main_hand_strength.straights());
    
}
