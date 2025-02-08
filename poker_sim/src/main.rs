mod models;

fn main() {
    let card_list = [models::Card {suit: 0, value: 0}; 52];

    let deck = models::Deck{cards: card_list};

    println!("{:?}", deck);
}
