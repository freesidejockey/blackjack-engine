use crate::card::{Card, Rank, Suit};
use crate::game::GameAction::{Double, Hit, Split, Stand};
use crate::game::GameState::WaitingToDeal;
use crate::game_settings::GameSettings;
use crate::hand::{Hand, HandOutcome};
use crate::player::Player;
use crate::shoe::Shoe;

pub struct Game {
    pub settings: GameSettings,
    pub shoe: Shoe,
    pub player: Player,
    pub dealer: Player,
    pub state: GameState,
}

impl Game {
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

    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    pub fn shuffle_shoe(&mut self) {
        self.shoe.shuffle();
    }

    pub fn accept_user_bet(&mut self, bet: f64) {
        if self.player.bank_roll < bet {
            println!("You cannot bet more than you have");
            return;
        }
        self.player.bank_roll -= bet;
        self.player.hands[0].bet = bet;
        self.state = WaitingToDeal { player_bet: bet, player_bankroll: self.player.bank_roll }
    }

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

        if self.player.hands[0].is_natural_blackjack() {
            if self.dealer.hands[0].is_natural_blackjack() {
                // push, add the bet back to player bankroll
                self.player.bank_roll += self.player.hands[0].bet;
                self.player.hands[0].outcome = Option::from(HandOutcome::Push);

                self.state = GameState::RoundComplete {
                    dealer_hand: self.dealer.hands[0].clone(),
                    player_hands: self.player.hands.clone(),
                    player_bankroll: self.player.bank_roll
                };
                return;
            } else {
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

        self.state = GameState::PlayerTurn {
            dealer_hand: self.dealer.hands[0].clone(),
            player_hands: self.player.hands.clone(),
            player_bankroll: self.player.bank_roll,
            active_hand_index: 0,
        }
    }

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
                    let mut new_hand = Hand::with_card_and_bet(split_card, new_bet);
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

    pub fn next_round(&mut self) {
        self.player.reset_hands();
        self.dealer.reset_hands();
        self.state = GameState::WaitingForBet { player_bankroll: self.player.bank_roll }
    }

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

pub enum PlayerType {
    MainPlayer,
    Dealer,
    Other(u32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameAction {
    Hit, Stand, Double, Split
}

impl GameAction {
    pub fn from_string(value: &str) -> Option<GameAction> {
        match value.to_lowercase().trim() {
            "h" | "hit" => Some(Hit),
            "s" | "stand" => Some(Stand),
            "d" | "double" => Some(Double),
            "p" | "split" => Some(Split),
            _ => None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Hit => "HIT".to_string(),
            Stand => "STAND".to_string(),
            Double => "DOUBLE".to_string(),
            Split => "SPLIT".to_string(),
        }
    }
}

pub enum HandResult {
    Blackjack, Bust, Other(Vec<i32>)
}

#[derive(PartialEq, Clone)]
pub enum GameState {
    WaitingForBet {
        player_bankroll: f64,
    },
    WaitingToDeal {
        player_bet: f64,
        player_bankroll: f64,
    },
    PlayerTurn {
        dealer_hand: Hand,
        player_hands: Vec<Hand>,
        player_bankroll: f64,
        active_hand_index: usize,
    },
    DealerTurn {
        dealer_hand: Hand,
        player_hands: Vec<Hand>,
        player_bankroll: f64,
    },
    RoundComplete {
        dealer_hand: Hand,
        player_hands: Vec<Hand>,
        player_bankroll: f64,
    }
}

#[derive(Clone)]
pub struct GameStateDTO {
    pub player_bankroll: f64,
}

impl GameStateDTO {
    pub fn new(player_bankroll: f64) -> Self {
        Self { player_bankroll}
    }
}