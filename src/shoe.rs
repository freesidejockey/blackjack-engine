use std::thread::sleep;
use std::time::Duration;
use strum::IntoEnumIterator;
use rand::seq::SliceRandom;
use crate::card::{Card, Rank, Suit};

/// Represents a dealer's shoe in a casino blackjack game.
///
/// A shoe contains multiple decks of cards and tracks both the active cards
/// and discarded cards. This implementation mirrors real casino practices
/// where multiple decks are shuffled together to make card counting more difficult.
pub struct Shoe {
    /// Cards currently available to be dealt
    pub cards: Vec<Card>,
    /// Cards that have been dealt and discarded
    pub discarded: Vec<Card>,
    /// Number of complete decks in the shoe
    number_of_decks: usize
}

impl Shoe {
    /// Creates a new shoe with the specified number of decks.
    ///
    /// The shoe is created with all cards in order (unshuffled). Cards
    /// are organized by rank and suit, repeated for each deck.
    ///
    /// # Arguments
    ///
    /// * `num_decks` - Number of standard 52-card decks to include in the shoe
    ///
    /// # Examples
    ///
    /// ```
    /// let shoe = Shoe::new(6); // Creates a 6-deck shoe (312 cards)
    /// assert_eq!(shoe.cards.len(), 312);
    /// ```
    pub fn new(num_decks: usize) -> Self {
        // Initialize a vector w/ size defined upfront
        let capacity = 52 * num_decks;
        let mut cards: Vec<Card> = Vec::with_capacity(capacity);

        for _ in 0..num_decks {
            cards.extend(
                Rank::iter()
                    .flat_map(|rank| {
                        Suit::iter().map(move |suit| Card::new(rank.clone(), suit))
                    })
                    .collect::<Vec<Card>>()
            );
        }

        Shoe {
            cards,
            discarded: Vec::with_capacity(capacity),
            number_of_decks: num_decks
        }
    }

    /// Shuffles all cards currently in the shoe.
    ///
    /// Uses the rand crate's thread_rng for secure random shuffling.
    /// This method only shuffles cards that haven't been dealt - it does
    /// not affect discarded cards.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut shoe = Shoe::new(1);
    /// shoe.shuffle(); // Randomizes order of cards
    /// ```
    pub fn shuffle(&mut self)  {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    /// Prints all cards currently in the shoe for debugging purposes.
    ///
    /// Displays each card's rank and suit on a new line.
    pub fn print_deck(&self) {
        for (_val, i) in self.cards.iter().enumerate() {
            print!("{}", i.rank.to_string());
            println!("{}", i.suit.to_string());
        }
    }

    /// Draws a single card from the top of the shoe.
    ///
    /// The drawn card is removed from the available cards and added to
    /// the discarded pile. Returns None if there are no cards left.
    ///
    /// # Returns
    ///
    /// * `Some(Card)` - The drawn card
    /// * `None` - If the shoe is empty
    ///
    /// # Examples
    ///
    /// ```
    /// let mut shoe = Shoe::new(1);
    /// if let Some(card) = shoe.draw_card() {
    ///     println!("Drew: {}", card.to_string());
    /// }
    /// ```
    pub fn draw_card(&mut self) -> Option<Card> {
        let card = self.cards.pop()?;
        self.discarded.push(card.clone());
        Some(card)
    }

    /// Ensures there are enough cards in the shoe for the current number of players.
    ///
    /// If there aren't enough cards remaining, creates and shuffles a new shoe.
    /// This simulates a dealer getting a new shoe when the current one runs low,
    /// which is standard casino practice.
    ///
    /// # Arguments
    ///
    /// * `num_players` - Current number of players at the table
    ///
    /// # Examples
    ///
    /// ```
    /// let mut shoe = Shoe::new(6);
    /// shoe.ensure_cards_for_players(3); // Ensures enough cards for 3 players
    /// ```
    ///
    /// # Notes
    ///
    /// Calculates minimum cards needed as:
    /// (num_players + 1 dealer) * 2 initial cards * 2 for potential additional draws
    pub fn ensure_cards_for_players(&mut self, num_players: usize) {
        // Calculate minimum cards needed:
        // (num_players + 1 for dealer) * 2 initial cards * 2 for potential additional draws
        let min_cards_needed = (num_players + 1) * 2 * 2;

        if self.cards.len() < min_cards_needed {
            // Create new shoe with calculated number of decks
            let new_shoe = Shoe::new(self.number_of_decks);
            self.cards = new_shoe.cards;
            self.discarded.clear();

            // Shuffle the new shoe
            self.shuffle();
            println!("Starting a new Shoe");
            sleep(Duration::from_millis(2000));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn test_shuffle_deck() {
        let mut deck = Shoe::new(4);
        let ordered_cards = deck.cards.clone();
        deck.shuffle();
        deck.shuffle();
        let shuffled_cards = deck.cards.clone();

        assert_ne!(ordered_cards, shuffled_cards);
    }

    #[test]
    fn test_shoe_creation() {
        let num_decks = 2;
        let shoe = Shoe::new(num_decks);
        assert_eq!(shoe.cards.len(), num_decks * 52);
    }

    #[test]
    fn test_draw_card() {
        let mut shoe = Shoe::new(1);
        let initial_count = shoe.cards.len();
        let card = shoe.draw_card();

        assert!(card.is_some());
        assert_eq!(shoe.cards.len(), initial_count - 1);
        assert_eq!(shoe.discarded.len(), 1);
    }

    #[test]
    fn test_draw_from_empty_shoe() {
        let mut shoe = Shoe::new(1);
        // Draw all cards
        while shoe.draw_card().is_some() {}

        assert!(shoe.draw_card().is_none());
        assert!(shoe.cards.is_empty());
        assert_eq!(shoe.discarded.len(), 52); // Full deck in discard
    }

    #[test]
    fn test_multiple_deck_size() {
        for num_decks in 1..=8 {
            let shoe = Shoe::new(num_decks);
            assert_eq!(shoe.cards.len(), num_decks * 52);
        }
    }

    #[test]
    fn test_new_shoe_has_all_cards() {
        let shoe = Shoe::new(1);
        let mut ranks = HashSet::new();
        let mut suits = HashSet::new();

        for card in &shoe.cards {
            ranks.insert(card.rank.clone());
            suits.insert(card.suit.clone());
        }

        assert_eq!(ranks.len(), 13); // All ranks present
        assert_eq!(suits.len(), 4);  // All suits present
    }
}