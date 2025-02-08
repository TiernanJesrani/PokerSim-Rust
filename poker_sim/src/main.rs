mod models;

fn main() {
    // let card_list = [models::Card {suit: 0, value: 0}; 52];

    let mut deck = models::Deck::new();

    deck.shuffle();

    println!("{:?}", deck);
}
