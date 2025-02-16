/// Represents the configuration settings for a blackjack game.
///
/// GameSettings holds all the configurable parameters that define how a blackjack
/// game should be set up and run. This includes player information and deck configuration.
/// The settings can be validated to ensure they meet game requirements.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use blackjack_engine::game_settings::GameSettings;
///
/// // Create settings for a standard 6-deck game
/// let settings = GameSettings::new("Alice".to_string(), 6);
///
/// // Validate the settings
/// assert!(settings.validate().is_ok());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GameSettings {
    /// Name of the main player. Must be non-empty when validated.
    pub player_name: String,
    /// Number of decks to use in the shoe. Valid range is 1-8 decks.
    pub deck_count: u8,
}

impl GameSettings {
    /// Creates a new GameSettings instance with the specified parameters.
    ///
    /// This method creates a new game configuration but does not validate
    /// the parameters. Call `validate()` separately to ensure the settings
    /// are valid.
    ///
    /// # Arguments
    /// * `player_name` - Name of the main player
    /// * `deck_count` - Number of decks to use (should be between 1 and 8)
    ///
    /// # Returns
    /// A new GameSettings instance
    ///
    /// # Examples
    ///
    /// ```
    /// use blackjack_engine::game_settings::GameSettings;
    ///
    /// // Create settings for a 6-deck game
    /// let settings = GameSettings::new(
    ///     "Alice".to_string(),
    ///     6,
    /// );
    /// assert_eq!(settings.player_name, "Alice");
    /// assert_eq!(settings.deck_count, 6);
    ///
    /// // Settings should be validated before use
    /// assert!(settings.validate().is_ok());
    /// ```
    pub fn new(player_name: String, deck_count: u8) -> Self {
        Self {
            player_name,
            deck_count,
        }
    }

    /// Creates a default single-player game configuration with 6 decks.
    ///
    /// This is a convenience method that creates a standard casino-style
    /// configuration with 6 decks. This is a common setup in many casinos
    /// and provides a good balance between game flow and card counting difficulty.
    ///
    /// # Arguments
    /// * `player_name` - Name of the main player
    ///
    /// # Returns
    /// A new GameSettings instance with default values
    ///
    /// # Examples
    ///
    /// ```
    /// use blackjack_engine::game_settings::GameSettings;
    ///
    /// let settings = GameSettings::default_single_player("Bob".to_string());
    /// assert_eq!(settings.deck_count, 6); // Always uses 6 decks
    /// assert_eq!(settings.player_name, "Bob");
    /// ```
    pub fn default_single_player(player_name: String) -> Self {
        Self {
            player_name,
            deck_count: 6,
        }
    }

    /// Validates if the settings are within acceptable ranges.
    ///
    /// This method checks:
    /// - Player name is not empty (after trimming whitespace)
    /// - Deck count is between 1 and 8 (inclusive)
    ///
    /// # Returns
    /// - `Ok(())` if all settings are valid
    /// - `Err(String)` with a description of the first validation error encountered
    ///
    /// # Examples
    ///
    /// ```
    /// use blackjack_engine::game_settings::GameSettings;
    ///
    /// // Valid settings
    /// let valid = GameSettings::new("Alice".to_string(), 6);
    /// assert!(valid.validate().is_ok());
    ///
    /// // Invalid: empty name
    /// let invalid = GameSettings::new("".to_string(), 6);
    /// assert_eq!(
    ///     invalid.validate().unwrap_err(),
    ///     "Player name cannot be empty"
    /// );
    ///
    /// // Invalid: too many decks
    /// let invalid = GameSettings::new("Alice".to_string(), 9);
    /// assert_eq!(
    ///     invalid.validate().unwrap_err(),
    ///     "Deck count must be between 1 and 8"
    /// );
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.player_name.trim().is_empty() {
            return Err("Player name cannot be empty".to_string());
        }
        if !(1..=8).contains(&self.deck_count) {
            return Err("Deck count must be between 1 and 8".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_settings() {
        let settings = GameSettings::new(
            "Player1".to_string(),
            6,
        );
        assert_eq!(settings.player_name, "Player1");
        assert_eq!(settings.deck_count, 6);
    }

    #[test]
    fn test_default_single_player() {
        let settings = GameSettings::default_single_player("Player1".to_string());
        assert_eq!(settings.player_name, "Player1");
        assert_eq!(settings.deck_count, 6);
    }

    #[test]
    fn test_validate_valid_settings() {
        let settings = GameSettings::new(
            "Player1".to_string(),
            6,
        );
        assert!(settings.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_name() {
        let settings = GameSettings::new(
            "".to_string(),
            6,
        );
        assert!(settings.validate().is_err());
        assert_eq!(
            settings.validate().unwrap_err(),
            "Player name cannot be empty"
        );
    }

    #[test]
    fn test_validate_deck_count() {
        let settings = GameSettings::new(
            "Player1".to_string(),
            9,
        );
        assert!(settings.validate().is_err());
        assert_eq!(
            settings.validate().unwrap_err(),
            "Deck count must be between 1 and 8"
        );
    }

    #[test]
    fn test_settings_clone_and_equality() {
        let settings1 = GameSettings::new(
            "Player1".to_string(),
            6,
        );
        let settings2 = settings1.clone();
        assert_eq!(settings1, settings2);
    }
}