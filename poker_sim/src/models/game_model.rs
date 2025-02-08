use crate::models::player_model::Player;
use crate::models::deck_model::Deck;

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub deck: Deck
}