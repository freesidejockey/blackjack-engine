use std::thread::sleep;
use std::time::Duration;
use strum::IntoEnumIterator;
use rand::seq::SliceRandom;
use crate::card::{Card, Rank, Suit};

pub struct Shoe {
    pub cards: Vec<Card>,
    pub discarded: Vec<Card>,
    number_of_decks: usize
    // Count will be implemented at a later date
    // pub count: i32,
}

impl Shoe {
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

    pub fn shuffle(&mut self)  {
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn print_deck(&self) {
        for (_val, i) in self.cards.iter().enumerate() {
            print!("{}", i.rank.to_string());
            println!("{}", i.suit.to_string());
        }
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        let card = self.cards.pop()?;
        self.discarded.push(card.clone());
        Some(card)
    }

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