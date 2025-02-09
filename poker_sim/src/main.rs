mod models;
mod tests;

fn main() {
    // let card_list = [models::Card {suit: 0, value: 0}; 52];

    //let mut deck = models::deck_model::Deck::new();

    //deck.remove_cards(false, 12, 0);

    //deck.shuffle();

    // while !deck.cards.is_empty() {
    //     println!("{:?}", deck.top_card());
    // }

    //println!("{:?}", deck);
    //u32 num_players, bool suited, u32 rank_1, u32 rank_2
    let mut game = models::game_model::Game::new(5, false, 5, 5);
    game.deal();

    game.flop();

    game.turn();

    game.river();

    game.form_hand_strengths();

    let pairs = game.main_hand_strength.pairs();

    let two_pairs = game.main_hand_strength.two_pairs();

    let sets = game.main_hand_strength.sets();

    println!("{:?}", pairs);

    println!("{:?}", two_pairs);

    println!("{:?}", sets);

    println!("{:?}", game.main_hand_strength.seven_cards);
    
}
