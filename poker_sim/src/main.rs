mod models;
mod tests;

fn main() {
    // let card_list = [models::Card {suit: 0, value: 0}; 52];

    let mut deck = models::deck_model::Deck::new();

    deck.remove_cards(false, 12, 0);

    //deck.shuffle();

    // while !deck.cards.is_empty() {
    //     println!("{:?}", deck.top_card());
    // }

    println!("{:?}", deck);
}
