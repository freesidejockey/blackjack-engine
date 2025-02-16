use serde::Serialize;
use strum_macros::EnumIter;

/// Represents the rank of a playing card.
///
/// The rank determines the card's value in games. Some ranks like the Ace
/// can have multiple values.
#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize)]
#[derive(EnumIter)]
pub enum Rank {
    Two, Three, Four, Five, Six, Seven, Eight,
    Nine, Ten, Jack, Queen, King, Ace,
}

impl Rank {
    /// Returns the possible numerical values for this rank.
    ///
    /// Most ranks have a single value, but Ace can be worth either 1 or 11
    /// (commonly used in games like Blackjack).
    ///
    /// # Examples
    ///
    /// ```
    /// use blackjack_engine::card::Rank;
    /// assert_eq!(Rank::Ace.value(), vec![1, 11]);
    /// assert_eq!(Rank::Ten.value(), vec![10]);
    /// ```
    pub fn value(&self) -> Vec<i32> {
        match self {
            Rank::Ace => vec![1, 11],
            Rank::Two => vec![2],
            Rank::Three => vec![3],
            Rank::Four => vec![4],
            Rank::Five => vec![5],
            Rank::Six => vec![6],
            Rank::Seven => vec![7],
            Rank::Eight => vec![8],
            Rank::Nine => vec![9],
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => vec![10],
        }
    }

    /// Converts the rank to its string representation.
    ///
    /// Face cards are represented by their first letter (J, Q, K),
    /// Ace is represented by 'A', and number cards by their number.
    ///
    /// # Examples
    ///
    /// ```
    /// use blackjack_engine::card::Rank;
    /// assert_eq!(Rank::Ace.to_string(), "A");
    /// assert_eq!(Rank::Ten.to_string(), "10");
    /// ```
    pub fn to_string(&self) -> String {
        match self {
            Rank::Ace => "A".to_string(),
            Rank::Two => "2".to_string(),
            Rank::Three => "3".to_string(),
            Rank::Four => "4".to_string(),
            Rank::Five => "5".to_string(),
            Rank::Six => "6".to_string(),
            Rank::Seven => "7".to_string(),
            Rank::Eight => "8".to_string(),
            Rank::Nine => "9".to_string(),
            Rank::Ten => "10".to_string(),
            Rank::Jack => "J".to_string(),
            Rank::Queen => "Q".to_string(),
            Rank::King => "K".to_string(),
        }
    }
}

/// Represents the suit of a playing card.
///
/// The four standard playing card suits: Clubs (♣️), Diamonds (♦️),
/// Hearts (❤️), and Spades (♠️).
#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize)]
#[derive(EnumIter)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    /// Converts the suit to its Unicode symbol representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use blackjack_engine::card::Suit;
    /// assert_eq!(Suit::Hearts.to_string(), "❤️");
    /// assert_eq!(Suit::Spades.to_string(), "♠️");
    /// ```
    pub fn to_string(&self) -> String {
        match self {
            Suit::Hearts => "❤️".to_string(),
            Suit::Diamonds => "♦️".to_string(),
            Suit::Clubs => "♣️".to_string(),
            Suit::Spades => "♠️".to_string(),
        }
    }
}

/// Represents a playing card with a rank and suit.
///
/// Each card combines a [`Rank`] and a [`Suit`] to create a unique card
/// in a standard 52-card deck.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    /// Creates a new card with the specified rank and suit.
    ///
    /// # Examples
    ///
    /// ```
    /// use blackjack_engine::card::{Card, Rank, Suit};
    /// let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);
    /// ```
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card {
            rank,
            suit,
        }
    }

    /// Returns a string representation of the card combining rank and suit symbols.
    ///
    /// # Examples
    ///
    /// ```
    /// use blackjack_engine::card::{Card, Rank, Suit};
    /// let card = Card::new(Rank::Ace, Suit::Clubs);
    /// assert_eq!(card.to_string(), "A♣️");
    /// ```
    pub fn to_string(&self) -> String {
        format!("{}{}", self.rank.to_string(), self.suit.to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_values() {
        assert_eq!(Rank::Ace.value(), vec![1, 11]);
        assert_eq!(Rank::Two.value(), vec![2]);
        assert_eq!(Rank::Three.value(), vec![3]);
        assert_eq!(Rank::Four.value(), vec![4]);
        assert_eq!(Rank::Five.value(), vec![5]);
        assert_eq!(Rank::Six.value(), vec![6]);
        assert_eq!(Rank::Seven.value(), vec![7]);
        assert_eq!(Rank::Eight.value(), vec![8]);
        assert_eq!(Rank::Nine.value(), vec![9]);
        assert_eq!(Rank::Ten.value(), vec![10]);
        assert_eq!(Rank::Jack.value(), vec![10]);
        assert_eq!(Rank::Queen.value(), vec![10]);
        assert_eq!(Rank::King.value(), vec![10]);
    }

    #[test]
    fn test_rank_to_string() {
        assert_eq!(Rank::Ace.to_string(), "A");
        assert_eq!(Rank::Two.to_string(), "2");
        assert_eq!(Rank::Three.to_string(), "3");
        assert_eq!(Rank::Four.to_string(), "4");
        assert_eq!(Rank::Five.to_string(), "5");
        assert_eq!(Rank::Six.to_string(), "6");
        assert_eq!(Rank::Seven.to_string(), "7");
        assert_eq!(Rank::Eight.to_string(), "8");
        assert_eq!(Rank::Nine.to_string(), "9");
        assert_eq!(Rank::Ten.to_string(), "10");
        assert_eq!(Rank::Jack.to_string(), "J");
        assert_eq!(Rank::Queen.to_string(), "Q");
        assert_eq!(Rank::King.to_string(), "K");
    }

    #[test]
    fn test_suit_to_string() {
        assert_eq!(Suit::Hearts.to_string(), "❤️");
        assert_eq!(Suit::Diamonds.to_string(), "♦️");
        assert_eq!(Suit::Clubs.to_string(), "♣️");
        assert_eq!(Suit::Spades.to_string(), "♠️");
    }

    #[test]
    fn test_card_to_string() {
        let card = Card::new(Rank::Ace, Suit::Clubs);
        assert_eq!(card.to_string(), "A♣️");
    }
}