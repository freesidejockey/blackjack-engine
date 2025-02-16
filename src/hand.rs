use serde::Serialize;
use strum_macros::EnumIter;
use crate::card::{Card, Rank};

/// Represents a player's hand in a blackjack game.
///
/// A hand contains cards, tracks the current bet amount, and records the outcome
/// of the hand once play is complete. The hand provides methods for calculating
/// values according to standard Blackjack rules, where aces can count as 1 or 11.
#[derive(PartialEq, Clone, Debug, Serialize)]
pub struct Hand {
    /// Current bet amount for this hand
    pub bet: f64,
    /// Cards in the hand
    pub cards: Vec<Card>,
    /// How the hand turned out (Win, Loss, Push, or Blackjack)
    pub outcome: Option<HandOutcome>
}

/// Represents the possible outcomes of a Blackjack hand.
///
/// * `Win` - Player won the hand
/// * `Loss` - Player lost the hand
/// * `Push` - Player tied with dealer
/// * `Blackjack` - Player got a natural blackjack (Ace + 10-value card)
#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize)]
#[derive(EnumIter)]
pub enum HandOutcome {
    Win,
    Loss,
    Push,
    Blackjack
}

impl HandOutcome {
    /// Converts the hand outcome to its string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use blackjack_engine::hand::HandOutcome;
    /// assert_eq!(HandOutcome::Blackjack.to_string(), "BLACKJACK");
    /// assert_eq!(HandOutcome::Push.to_string(), "PUSH");
    /// ```
    pub fn to_string(&self) -> String {
        match self {
            HandOutcome::Win => "WIN".to_string(),
            HandOutcome::Loss => "LOSS".to_string(),
            HandOutcome::Push => "PUSH".to_string(),
            HandOutcome::Blackjack => "BLACKJACK".to_string(),
        }
    }
}

impl Hand {
    /// Creates a new empty hand with no bet.
    ///
    /// # Examples
    ///
    /// ```
    /// let hand = Hand::new();
    /// assert_eq!(hand.bet, 0f64);
    /// assert!(hand.cards.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            bet: 0f64,
            cards: Vec::new(),
            outcome: None
        }
    }

    /// Creates a new empty hand with a specific bet amount.
    ///
    /// # Examples
    ///
    /// ```
    /// let hand = Hand::with_bet(100.0);
    /// assert_eq!(hand.bet, 100.0);
    /// ```
    pub fn with_bet(bet: f64) -> Self {
        Self {
            bet,
            cards: Vec::new(),
            outcome: None
        }
    }

    /// Creates a new hand with a single card and no bet.
    ///
    /// # Examples
    ///
    /// ```
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// let hand = Hand::with_card(card);
    /// assert_eq!(hand.cards.len(), 1);
    /// ```
    pub fn with_card(card: Card) -> Self {
        Self {
            bet: 0f64,
            cards: vec![card],
            outcome: None
        }
    }

    /// Creates a new hand with a single card and specified bet amount.
    ///
    /// # Examples
    ///
    /// ```
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// let hand = Hand::with_card_and_bet(card, 100.0);
    /// assert_eq!(hand.cards.len(), 1);
    /// assert_eq!(hand.bet, 100.0);
    /// ```
    pub fn with_card_and_bet(card: Card, bet: f64) -> Self {
        Self {
            bet,
            cards: vec![card],
            outcome: None
        }
    }

    /// Adds a card to the hand.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut hand = Hand::new();
    /// hand.add_card(Card::new(Rank::Ace, Suit::Spades));
    /// assert_eq!(hand.cards.len(), 1);
    /// ```
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Returns true if the hand can be split (has exactly 2 cards of the same rank).
    ///
    /// This is used to determine if the player can split their hand according to
    /// standard Blackjack rules.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut hand = Hand::new();
    /// hand.add_card(Card::new(Rank::Eight, Suit::Spades));
    /// hand.add_card(Card::new(Rank::Eight, Suit::Hearts));
    /// assert!(hand.can_split());
    /// ```
    pub fn can_split(&self) -> bool {
        self.cards.len() == 2 && self.cards[0].rank == self.cards[1].rank
    }

    /// Doubles the current bet amount.
    ///
    /// This is typically used when the player chooses to "double down" on their hand.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut hand = Hand::with_bet(100.0);
    /// hand.double_bet();
    /// assert_eq!(hand.bet, 200.0);
    /// ```
    pub fn double_bet(&mut self) {
        self.bet = self.bet * 2f64;
    }

    /// Calculates all possible hand values, accounting for aces.
    ///
    /// Returns a vector of possible totals, sorted from lowest to highest. Aces can
    /// count as 1 or 11, so hands with aces will have multiple possible values.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut hand = Hand::new();
    /// hand.add_card(Card::new(Rank::Ace, Suit::Spades));
    /// hand.add_card(Card::new(Rank::Ace, Suit::Hearts));
    /// assert_eq!(hand.possible_values(), vec![2, 12, 22]);
    /// ```
    pub fn possible_values(&self) -> Vec<u32> {
        let (non_ace_total, ace_count) = self.cards.iter().fold((0, 0), |(total, aces), card| {
            match card.rank {
                Rank::Ace => (total, aces + 1),
                _ => (total + card.rank.value()[0] as u32, aces),
            }
        });

        let mut totals = vec![non_ace_total];

        // Add possible ace values
        for _ in 0..ace_count {
            let mut new_totals = Vec::new();
            for total in totals.iter() {
                new_totals.push(*total + 1);  // Ace counted as 1
                new_totals.push(*total + 11);  // Ace counted as 11
            }
            totals = new_totals;
        }

        totals.sort_unstable();
        totals.dedup();
        totals
    }

    /// Returns the best (highest non-busting) total for the hand.
    ///
    /// If all totals would bust (exceed 21), returns the lowest total.
    /// This follows standard Blackjack rules where aces can be worth 1 or 11,
    /// and you want the highest possible total that doesn't bust.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut hand = Hand::new();
    /// hand.add_card(Card::new(Rank::Ace, Suit::Spades));
    /// hand.add_card(Card::new(Rank::King, Suit::Hearts));
    /// assert_eq!(hand.best_value(), 21);
    /// ```
    pub fn best_value(&self) -> u32 {
        let values = self.possible_values();
        values.iter()
            .rev()
            .find(|&&v| v <= 21)
            .copied()
            .unwrap_or_else(|| values[0])
    }

    /// Returns true if the hand is a natural blackjack (21 with exactly 2 cards).
    ///
    /// A natural blackjack typically pays out at higher odds than other wins.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut hand = Hand::new();
    /// hand.add_card(Card::new(Rank::Ace, Suit::Spades));
    /// hand.add_card(Card::new(Rank::King, Suit::Hearts));
    /// assert!(hand.is_natural_blackjack());
    /// ```
    pub fn is_natural_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.best_value() == 21
    }

    /// Returns true if the hand totals 21 (not necessarily a natural blackjack).
    ///
    /// This differs from `is_natural_blackjack()` in that it returns true for any
    /// combination of cards that total 21, not just two-card combinations.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut hand = Hand::new();
    /// hand.add_card(Card::new(Rank::Five, Suit::Spades));
    /// hand.add_card(Card::new(Rank::Six, Suit::Hearts));
    /// hand.add_card(Card::new(Rank::King, Suit::Diamonds));
    /// assert!(hand.is_blackjack());
    /// assert!(!hand.is_natural_blackjack());
    /// ```
    pub fn is_blackjack(&self) -> bool {
        self.best_value() == 21
    }

    /// Returns true if the hand is busted (all possible totals exceed 21).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut hand = Hand::new();
    /// hand.add_card(Card::new(Rank::King, Suit::Spades));
    /// hand.add_card(Card::new(Rank::Queen, Suit::Hearts));
    /// hand.add_card(Card::new(Rank::Jack, Suit::Diamonds));
    /// assert!(hand.is_busted());
    /// ```
    pub fn is_busted(&self) -> bool {
        self.possible_values().iter().all(|&v| v > 21)
    }

    /// Returns a string representation of the hand, showing all cards.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut hand = Hand::new();
    /// hand.add_card(Card::new(Rank::Ace, Suit::Spades));
    /// hand.add_card(Card::new(Rank::King, Suit::Hearts));
    /// assert_eq!(hand.to_string(), "A♠️ K❤️ ");
    /// ```
    pub fn to_string(&self) -> String {
        let mut hand_as_string = String::from("");
        for card in self.cards.iter() {
            hand_as_string.push_str(&*card.to_string());
            hand_as_string.push(' ');
        }
        hand_as_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Suit;

    #[test]
    fn test_new_hand() {
        let hand = Hand::new();
        assert_eq!(hand.bet, 0f64);
        assert!(hand.cards.is_empty());
    }

    #[test]
    fn test_with_bet() {
        let hand = Hand::with_bet(200f64);
        assert_eq!(hand.bet, 200f64);
    }

    #[test]
    fn test_add_card() {
        let mut hand = Hand::new();
        let card = Card::new(Rank::Ace, Suit::Spades);
        hand.add_card(card);
        assert_eq!(hand.cards.len(), 1);
    }

    #[test]
    fn test_possible_values() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spades));
        hand.add_card(Card::new(Rank::Ace, Suit::Hearts));
        let values = hand.possible_values();
        assert_eq!(values, vec![2, 12, 22]);
    }

    #[test]
    fn test_best_value() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spades));
        hand.add_card(Card::new(Rank::King, Suit::Hearts));
        assert_eq!(hand.best_value(), 21);
    }

    #[test]
    fn test_blackjack() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spades));
        hand.add_card(Card::new(Rank::King, Suit::Hearts));
        assert!(hand.is_blackjack());
    }

    #[test]
    fn test_bust() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::King, Suit::Spades));
        hand.add_card(Card::new(Rank::Queen, Suit::Hearts));
        hand.add_card(Card::new(Rank::Jack, Suit::Diamonds));
        assert!(hand.is_busted());
    }
}