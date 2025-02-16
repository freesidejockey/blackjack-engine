pub mod card;
pub mod shoe;
pub mod player;
pub mod hand;
pub mod game_settings;
pub mod game;

pub fn add_one(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2, 2);
        assert_eq!(result, 4);
    }
}
