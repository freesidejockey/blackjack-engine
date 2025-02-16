#[derive(Debug, Clone, PartialEq)]
pub struct GameSettings {
    /// Name of the main player
    pub player_name: String,
    /// Number of decks to use in the shoe
    pub deck_count: u8,
}

impl GameSettings {
    /// Creates a new GameSettings instance with the specified parameters
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
    /// let settings = GameSettings::new(
    ///     "Alice".to_string(),
    ///     6,
    /// );
    /// assert_eq!(settings.player_name, "Alice");
    /// assert_eq!(settings.deck_count, 6);
    /// ```
    pub fn new(player_name: String, deck_count: u8) -> Self {
        Self {
            player_name,
            deck_count,
        }
    }

    /// Creates a default single-player game configuration with 6 decks
    ///
    /// # Arguments
    /// * `player_name` - Name of the main player
    ///
    /// # Returns
    /// A new GameSettings instance with default values
    pub fn default_single_player(player_name: String) -> Self {
        Self {
            player_name,
            deck_count: 6,
        }
    }

    /// Validates if the settings are within acceptable ranges
    ///
    /// # Returns
    /// `Ok(())` if settings are valid, `Err` with description if invalid
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
            1,
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