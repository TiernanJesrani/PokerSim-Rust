use crate::models::card_model::Card;

#[derive(Debug, Clone)]
pub struct Player {
    pub hand: Vec<Card>
}