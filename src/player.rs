use crate::card::Card;
use crate::hand::Hand;

/// Represents a player in a blackjack game.
///
/// A player can have multiple hands (due to splits) and maintains a bankroll
/// to track their available funds. The player structure manages the state
/// of all active hands and their total money.
pub struct Player {
    /// The player's active hands. Usually just one hand, but can have multiple after splitting.
    pub hands: Vec<Hand>,
    /// The player's available money for betting
    pub bank_roll: f64
    // I'll eventually want to track previous_hands, but not necessary yet
    // pub previous_hands: Vec<Hand>,
}

impl Player {
    /// Creates a new player with a default bankroll of 10,000.
    ///
    /// The player starts with one empty hand and the standard starting bankroll.
    ///
    /// # Examples
    ///
    /// ```
    /// use blackjack_engine::player::Player;
    /// let player = Player::new();
    /// assert_eq!(player.bank_roll, 10_000f64);
    /// assert_eq!(player.hands.len(), 1);
    /// ```
    pub fn new() -> Player {
        Player {
            hands: vec![Hand::new()],
            bank_roll: 10_000f64
        }
    }

    /// Creates a new player with a specified bankroll amount.
    ///
    /// The player starts with one empty hand and the provided bankroll.
    ///
    /// # Examples
    ///
    /// ```
    /// use blackjack_engine::player::Player;
    /// let player = Player::with_bankroll(5_000f64);
    /// assert_eq!(player.bank_roll, 5_000f64);
    /// ```
    pub fn with_bankroll(bankroll: f64) -> Player {
        Player {
            hands: vec![Hand::new()],
            bank_roll: bankroll
        }
    }

    /// Adds a card to the specified hand.
    ///
    /// If the hand_index is invalid (i.e., the player doesn't have that many hands),
    /// the card will not be added and the operation will silently fail.
    ///
    /// # Arguments
    ///
    /// * `card` - The card to add to the hand
    /// * `hand_index` - The index of the hand to add the card to (0-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use blackjack_engine::card::{Card, Rank, Suit};
    /// use blackjack_engine::player::Player;
    /// let mut player = Player::new();
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// player.add_card_to_hand(card, 0); // Adds to first hand
    /// assert_eq!(player.hands[0].cards.len(), 1);
    /// ```
    pub fn add_card_to_hand(&mut self, card: Card, hand_index: usize) {
        if let Some(hand) = self.hands.get_mut(hand_index) {
            hand.add_card(card);
        }
    }

    /// Resets the player's hands to a single empty hand.
    ///
    /// This is typically called at the end of a round to prepare for the next hand.
    /// The bankroll remains unchanged, but all current hands are cleared and reset
    /// to a single empty hand.
    ///
    /// # Examples
    ///
    /// ```
    /// use blackjack_engine::card::{Card, Rank, Suit};
    /// use blackjack_engine::player::Player;
    /// let mut player = Player::new();
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// player.add_card_to_hand(card, 0);
    /// player.reset_hands();
    /// assert_eq!(player.hands[0].cards.len(), 0);
    /// ```
    pub fn reset_hands(&mut self) {
        self.hands = vec![Hand::new()]
    }

    /// Prints the current state of all active hands to the console.
    ///
    /// Each hand is numbered starting from 1, and all cards in each hand
    /// are displayed using their string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use blackjack_engine::card::{Card, Rank, Suit};
    /// use blackjack_engine::player::Player;
    /// let mut player = Player::new();
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// player.add_card_to_hand(card, 0);
    /// player.print_active_hand(); // Outputs: "Hand 1: A♠️ "
    /// ```
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