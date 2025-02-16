use serde::Serialize;
use crate::game::GameAction::{Double, Hit, Split, Stand};
use crate::game::GameState::WaitingToDeal;
use crate::game_settings::GameSettings;
use crate::hand::{Hand, HandOutcome};
use crate::player::Player;
use crate::shoe::Shoe;

/// Represents a blackjack game instance.
///
/// The Game struct manages the entire state of a blackjack game, including
/// players, cards, and game progression. It implements standard casino
/// blackjack rules and handles all game actions and state transitions.
pub struct Game {
    /// Configuration settings for the game
    pub settings: GameSettings,
    /// The shoe containing all cards for the game
    pub shoe: Shoe,
    /// The main player
    pub player: Player,
    /// The dealer
    pub dealer: Player,
    /// Current state of the game
    pub state: GameState,
}

impl Game {
    /// Creates a new blackjack game with the specified settings.
    ///
    /// # Arguments
    ///
    /// * `settings` - Configuration settings for the game
    ///
    /// # Examples
    ///
    /// ```
    /// use blackjack_engine::game::Game;
    /// use blackjack_engine::game_settings::GameSettings;
    /// let settings = GameSettings::default_single_player("Player1".to_string());
    /// let game = Game::new(settings);
    /// ```
    pub fn new(settings: GameSettings) -> Game {
        let player = Player::new();
        let bankroll = player.bank_roll;
        Game {
            player,
            dealer: Player::new(),
            shoe: Shoe::new(settings.deck_count as usize),
            settings,
            state: GameState::WaitingForBet { player_bankroll: bankroll }
        }
    }

    /// Returns a reference to the current game state.
    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    /// Shuffles all cards in the shoe.
    pub fn shuffle_shoe(&mut self) {
        self.shoe.shuffle();
    }

    /// Processes a player's bet attempt.
    ///
    /// Validates that the player has sufficient funds and updates the game
    /// state accordingly. If the bet is invalid, prints an error message
    /// and maintains the current state.
    ///
    /// # Arguments
    ///
    /// * `bet` - The amount the player wants to bet
    pub fn accept_user_bet(&mut self, bet: f64) {
        if self.player.bank_roll < bet {
            println!("You cannot bet more than you have");
            return;
        }
        self.player.bank_roll -= bet;
        self.player.hands[0].bet = bet;
        self.state = WaitingToDeal { player_bet: bet, player_bankroll: self.player.bank_roll }
    }

    /// Deals the initial two cards to both player and dealer.
    ///
    /// This method:
    /// 1. Ensures sufficient cards are available
    /// 2. Deals alternating cards to player and dealer
    /// 3. Checks for natural blackjacks
    /// 4. Updates game state based on initial hands
    pub fn deal_initial_cards(&mut self) {
        // Deal two cards to player and dealer
        self.shoe.ensure_cards_for_players(1);
        for _ in 0..2 {
            if let Some(card) = self.shoe.draw_card() {
                self.player.add_card_to_hand(card, 0);
            }
            if let Some(card) = self.shoe.draw_card() {
                self.dealer.add_card_to_hand(card, 0);
            }
        }

        // Handle natural blackjacks
        if self.player.hands[0].is_natural_blackjack() {
            if self.dealer.hands[0].is_natural_blackjack() {
                // Push - return bet to player
                self.player.bank_roll += self.player.hands[0].bet;
                self.player.hands[0].outcome = Option::from(HandOutcome::Push);
                self.state = GameState::RoundComplete {
                    dealer_hand: self.dealer.hands[0].clone(),
                    player_hands: self.player.hands.clone(),
                    player_bankroll: self.player.bank_roll
                };
                return;
            } else {
                // Player blackjack pays 3:2
                self.player.bank_roll += self.player.hands[0].bet * 2.5;
                self.player.hands[0].outcome = Option::from(HandOutcome::Blackjack);
                self.state = GameState::RoundComplete {
                    dealer_hand: self.dealer.hands[0].clone(),
                    player_hands: self.player.hands.clone(),
                    player_bankroll: self.player.bank_roll
                };
                return;
            }
        }

        if self.dealer.hands[0].is_natural_blackjack() {
            self.player.hands[0].outcome = Option::from(HandOutcome::Loss);
            self.state = GameState::RoundComplete {
                dealer_hand: self.dealer.hands[0].clone(),
                player_hands: self.player.hands.clone(),
                player_bankroll: self.player.bank_roll
            };
            return;
        }

        // No blackjacks - proceed to player's turn
        self.state = GameState::PlayerTurn {
            dealer_hand: self.dealer.hands[0].clone(),
            player_hands: self.player.hands.clone(),
            player_bankroll: self.player.bank_roll,
            active_hand_index: 0,
        }
    }

    /// Processes a player's action during their turn.
    ///
    /// # Arguments
    ///
    /// * `action` - The action chosen by the player (Hit, Stand, Double, or Split)
    /// * `hand_index` - Index of the hand being played (relevant for split hands)
    ///
    /// Handles all possible player actions including:
    /// - Hit: Draw another card
    /// - Stand: End turn for current hand
    /// - Double: Double bet and take one card
    /// - Split: Split matching cards into two hands
    pub fn process_player_action(&mut self, action: GameAction, hand_index: usize) {
        match action {
            Hit => {
                if let Some(card) = self.shoe.draw_card() {
                    self.player.add_card_to_hand(card, hand_index);
                    if self.player.hands[hand_index].is_busted() {
                        self.player.hands[hand_index].outcome = Option::from(HandOutcome::Loss);
                        if self.player.hands.len() > hand_index + 1 {
                            // If there is another hand, it was split and needs at least one
                            // more card
                            if let Some(card) = self.shoe.draw_card() {
                                self.player.add_card_to_hand(card, hand_index + 1);
                            }
                            self.state = GameState::PlayerTurn {
                                dealer_hand: self.dealer.hands[0].clone(),
                                player_hands: self.player.hands.clone(),
                                player_bankroll: self.player.bank_roll,
                                active_hand_index: hand_index + 1
                            };
                            return;
                        }
                        self.state = GameState::RoundComplete {
                            dealer_hand: self.dealer.hands[0].clone(),
                            player_hands: self.player.hands.clone(),
                            player_bankroll: self.player.bank_roll
                        };
                        return;
                    }

                    if self.player.hands[hand_index].is_blackjack() {
                        if self.player.hands.len() > hand_index + 1 {
                            // If there is another hand, it was split and needs at least one
                            // more card
                            if let Some(card) = self.shoe.draw_card() {
                                self.player.add_card_to_hand(card, hand_index + 1);
                            }
                            self.state = GameState::PlayerTurn {
                                dealer_hand: self.dealer.hands[0].clone(),
                                player_hands: self.player.hands.clone(),
                                player_bankroll: self.player.bank_roll,
                                active_hand_index: hand_index + 1
                            };
                            return;
                        }
                        self.state = GameState::DealerTurn{
                            dealer_hand: self.dealer.hands[0].clone(),
                            player_hands: self.player.hands.clone(),
                            player_bankroll: self.player.bank_roll
                        };
                        return;
                    }

                    self.state = GameState::PlayerTurn {
                        dealer_hand: self.dealer.hands[0].clone(),
                        player_hands: self.player.hands.clone(),
                        player_bankroll: self.player.bank_roll,
                        active_hand_index: hand_index
                    }
                }
            },
            Stand => {
                if self.player.hands.len() > hand_index + 1 {
                    // If there is another hand, it was split and needs at least one
                    // more card
                    if let Some(card) = self.shoe.draw_card() {
                        self.player.add_card_to_hand(card, hand_index + 1);
                    }
                    self.state = GameState::PlayerTurn {
                        dealer_hand: self.dealer.hands[0].clone(),
                        player_hands: self.player.hands.clone(),
                        player_bankroll: self.player.bank_roll,
                        active_hand_index: hand_index + 1
                    };
                    return;
                }
                self.state = GameState::DealerTurn {
                    dealer_hand: self.dealer.hands[0].clone(),
                    player_hands: self.player.hands.clone(),
                    player_bankroll: self.player.bank_roll
                }
            }
            Double => {
                if let Some(card) = self.shoe.draw_card() {
                    self.player.add_card_to_hand(card, hand_index);
                    self.player.bank_roll -= self.player.hands[hand_index].bet;
                    self.player.hands[hand_index].bet = self.player.hands[hand_index].bet * 2f64;
                    if self.player.hands.len() > hand_index + 1 {
                        if let Some(card) = self.shoe.draw_card() {
                            self.player.add_card_to_hand(card, hand_index + 1);
                        }
                        self.state = GameState::PlayerTurn {
                            dealer_hand: self.dealer.hands[0].clone(),
                            player_hands: self.player.hands.clone(),
                            player_bankroll: self.player.bank_roll,
                            active_hand_index: hand_index + 1
                        };
                        return;
                    }
                    self.state = GameState::DealerTurn {
                        dealer_hand: self.dealer.hands[0].clone(),
                        player_hands: self.player.hands.clone(),
                        player_bankroll: self.player.bank_roll
                    }
                }
            },
            Split => {
                // Check if we can split (should have exactly 2 equal cards)
                if self.player.hands[hand_index].cards.len() == 2
                    && self.player.hands[hand_index].cards[hand_index].rank == self.player.hands[hand_index].cards[1].rank {
                    // Take second card from first hand
                    let split_card = self.player.hands[hand_index].cards.pop().unwrap();

                    // Create new hand with the split card and same bet
                    let new_bet = self.player.hands[hand_index].bet;
                    self.player.bank_roll -= new_bet;  // Deduct additional bet for new hand

                    // Add second hand with split card at index + 1
                    let new_hand = Hand::with_card_and_bet(split_card, new_bet);
                    self.player.hands.insert(hand_index + 1, new_hand);

                    // Draw a card for the first hand only
                    if let Some(card1) = self.shoe.draw_card() {
                        self.player.add_card_to_hand(card1, hand_index);
                        self.state = GameState::PlayerTurn {
                            dealer_hand: self.dealer.hands[0].clone(),
                            player_hands: self.player.hands.clone(),
                            player_bankroll: self.player.bank_roll,
                            active_hand_index: hand_index
                        }
                    }
                }
            }
        }
    }

    /// Processes the dealer's turn according to standard casino rules.
    ///
    /// The dealer must:
    /// - Hit on 16 or below
    /// - Stand on 17 or above
    /// - Continue until reaching 17+ or busting
    pub fn next_dealer_turn(&mut self) {
        match self.state {
            GameState::DealerTurn { dealer_hand: _, player_hands: _, player_bankroll: _, .. } => {
                let dealer_value = self.dealer.hands[0].best_value();

                // Dealer must hit on 16 or below
                if dealer_value <= 16 {
                    if let Some(card) = self.shoe.draw_card() {
                        self.dealer.add_card_to_hand(card, 0);

                        // Check if dealer busted
                        if self.dealer.hands[0].is_busted() {
                            self.determine_winner_and_complete_round();
                            return;
                        }

                        // Continue dealer's turn
                        self.state = GameState::DealerTurn {
                            dealer_hand: self.dealer.hands[0].clone(),
                            player_hands: self.player.hands.clone(),
                            player_bankroll: self.player.bank_roll
                        };
                    }
                } else {
                    self.determine_winner_and_complete_round();
                }
            },
            _ => {
            }
        }
    }

    /// Prepares the game for a new round.
    ///
    /// Resets all hands and returns to the betting state.
    pub fn next_round(&mut self) {
        self.player.reset_hands();
        self.dealer.reset_hands();
        self.state = GameState::WaitingForBet { player_bankroll: self.player.bank_roll }
    }

    /// Determines the winner(s) and updates player bankroll accordingly.
    ///
    /// Compares dealer and player hand values according to standard blackjack rules:
    /// - Dealer bust: All non-busted player hands win
    /// - Otherwise: Higher hand value wins
    /// - Equal values: Push (tie)
    pub fn determine_winner_and_complete_round(&mut self) {
        let dealer_hand = &self.dealer.hands[0];
        let dealer_value = dealer_hand.best_value();
        for (_, hand) in self.player.hands.iter_mut().enumerate() {
            let player_value = hand.best_value();
            let hand_outcome = if hand.is_busted() {
                HandOutcome::Loss
            } else if dealer_hand.is_busted() {
                self.player.bank_roll += hand.bet * 2f64;
                HandOutcome::Win
            } else if dealer_value > player_value {
                HandOutcome::Loss
            } else if player_value > dealer_value {
                self.player.bank_roll += hand.bet * 2f64;
                HandOutcome::Win
            } else {
                self.player.bank_roll += hand.bet;
                HandOutcome::Push
            };
            hand.outcome = Option::from(hand_outcome);
        }

        self.state = GameState::RoundComplete {
            dealer_hand: self.dealer.hands[0].clone(),
            player_hands: self.player.hands.clone(),
            player_bankroll: self.player.bank_roll
        };
        return;
    }
}

/// Represents possible actions a player can take during their turn.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameAction {
    Hit,
    Stand,
    Double,
    Split
}

impl GameAction {
    /// Converts a string input to a GameAction.
    ///
    /// # Arguments
    ///
    /// * `value` - The string to convert
    ///
    /// # Returns
    ///
    /// Option<GameAction> - Some(action) if valid input, None if invalid
    pub fn from_string(value: &str) -> Option<GameAction> {
        match value.to_lowercase().trim() {
            "h" | "hit" => Some(Hit),
            "s" | "stand" => Some(Stand),
            "d" | "double" => Some(Double),
            "p" | "split" => Some(Split),
            _ => None
        }
    }

    /// Converts the action to its string representation.
    pub fn to_string(&self) -> String {
        match self {
            Hit => "HIT".to_string(),
            Stand => "STAND".to_string(),
            Double => "DOUBLE".to_string(),
            Split => "SPLIT".to_string(),
        }
    }
}

/// Represents the current state of the game.
#[derive(PartialEq, Clone)]
pub enum GameState {
    /// Waiting for player to place initial bet
    WaitingForBet {
        player_bankroll: f64,
    },
    /// Bet placed, waiting to deal cards
    WaitingToDeal {
        player_bet: f64,
        player_bankroll: f64,
    },
    /// Player's turn to act
    PlayerTurn {
        dealer_hand: Hand,
        player_hands: Vec<Hand>,
        player_bankroll: f64,
        active_hand_index: usize,
    },
    /// Dealer's turn to act
    DealerTurn {
        dealer_hand: Hand,
        player_hands: Vec<Hand>,
        player_bankroll: f64,
    },
    /// Round is complete, showing results
    RoundComplete {
        dealer_hand: Hand,
        player_hands: Vec<Hand>,
        player_bankroll: f64,
    }
}

/// Represents the complete game state with optional fields
/// depending on the current phase of the game.
#[derive(Clone, Debug, Serialize)]  // Add serde for JSON serialization
pub struct GameStateDto {
    /// Current phase of the game
    pub phase: GamePhase,
    /// Player's current bankroll
    pub player_bankroll: f64,
    /// Current bet amount, if a bet has been placed
    pub player_bet: Option<f64>,
    /// Dealer's hand, if cards have been dealt
    pub dealer_hand: Option<Hand>,
    /// Player's hands (multiple possible due to splits)
    pub player_hands: Option<Vec<Hand>>,
    /// Index of the active hand (relevant during player turns)
    pub active_hand_index: Option<usize>,
}

/// Represents the current phase of the game
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum GamePhase {
    WaitingForBet,
    WaitingToDeal,
    PlayerTurn,
    DealerTurn,
    RoundComplete,
}

impl From<GameState> for GameStateDto {
    fn from(state: GameState) -> Self {
        match state {
            GameState::WaitingForBet { player_bankroll } => GameStateDto {
                phase: GamePhase::WaitingForBet,
                player_bankroll,
                player_bet: None,
                dealer_hand: None,
                player_hands: None,
                active_hand_index: None,
            },
            GameState::WaitingToDeal { player_bet, player_bankroll } => GameStateDto {
                phase: GamePhase::WaitingToDeal,
                player_bankroll,
                player_bet: Some(player_bet),
                dealer_hand: None,
                player_hands: None,
                active_hand_index: None,
            },
            GameState::PlayerTurn { dealer_hand, player_hands, player_bankroll, active_hand_index } => GameStateDto {
                phase: GamePhase::PlayerTurn,
                player_bankroll,
                player_bet: player_hands.first().map(|h| h.bet),
                dealer_hand: Some(dealer_hand),
                player_hands: Some(player_hands),
                active_hand_index: Some(active_hand_index),
            },
            GameState::DealerTurn { dealer_hand, player_hands, player_bankroll } => GameStateDto {
                phase: GamePhase::DealerTurn,
                player_bankroll,
                player_bet: player_hands.first().map(|h| h.bet),
                dealer_hand: Some(dealer_hand),
                player_hands: Some(player_hands),
                active_hand_index: None,
            },
            GameState::RoundComplete { dealer_hand, player_hands, player_bankroll } => GameStateDto {
                phase: GamePhase::RoundComplete,
                player_bankroll,
                player_bet: player_hands.first().map(|h| h.bet),
                dealer_hand: Some(dealer_hand),
                player_hands: Some(player_hands),
                active_hand_index: None,
            },
        }
    }
}