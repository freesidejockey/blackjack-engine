use crate::card::Card;
use crate::hand::Hand;

pub struct Player {
    pub hands: Vec<Hand>,
    pub bank_roll: f64
    // I'll eventually want to track previous_hands, but not necessary yet
    // pub previous_hands: Vec<Hand>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            hands: vec![Hand::new()],
            bank_roll: 10_000f64
        }
    }

    pub fn add_card_to_hand(&mut self, card: Card, hand_index: usize) {
        if let Some(hand) = self.hands.get_mut(hand_index) {
            hand.add_card(card);
        }
    }

    pub fn reset_hands(&mut self) {
        self.hands = vec![Hand::new()]
    }

    pub fn print_active_hand(&self) {
        for (i, hand) in self.hands.iter().enumerate() {
            print!("Hand {}: ", i + 1);
            for card in hand.cards.iter() {
                print!("{} ", card.to_string());
            }
            println!("\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};

    #[test]
    fn test_new_player() {
        let player = Player::new();
        assert_eq!(player.hands.len(), 1);
        assert_eq!(player.bank_roll, 10_000f64);
        assert_eq!(player.hands[0].cards.len(), 0);
    }

    #[test]
    fn test_add_card_to_hand() {
        let mut player = Player::new();
        let card = Card::new(Rank::Ace, Suit::Spades);
        player.add_card_to_hand(card, 0);

        assert_eq!(player.hands[0].cards.len(), 1);
        assert_eq!(player.hands[0].cards[0].rank, Rank::Ace);
        assert_eq!(player.hands[0].cards[0].suit, Suit::Spades);
    }

    #[test]
    fn test_add_card_to_invalid_hand() {
        let mut player = Player::new();
        let card = Card::new(Rank::Ace, Suit::Spades);
        player.add_card_to_hand(card, 999); // Invalid index

        assert_eq!(player.hands[0].cards.len(), 0); // Should not add card
    }

    #[test]
    fn test_add_multiple_cards() {
        let mut player = Player::new();
        let card1 = Card::new(Rank::Ace, Suit::Spades);
        let card2 = Card::new(Rank::King, Suit::Hearts);

        player.add_card_to_hand(card1, 0);
        player.add_card_to_hand(card2, 0);

        assert_eq!(player.hands[0].cards.len(), 2);
    }

    #[test]
    fn test_reset_hands() {
        let mut player = Player::new();
        let card1 = Card::new(Rank::Ace, Suit::Spades);

        player.add_card_to_hand(card1, 0);

        assert_eq!(player.hands[0].cards.len(), 1);
        player.reset_hands();
        assert_eq!(player.hands[0].cards.len(), 0);
    }
}