use crate::enumord::EnumOrdering;

// Constant defining the maximum number of players
pub const ENUM_MAXPLAYERS: usize = 12;

use serde::{Deserialize, Serialize};

// Enumeration of different poker variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Game {
    Holdem,
    Holdem8,
    Omaha,
    Omaha5,
    Omaha6,
    Omaha8,
    Omaha85,
    Stud7,
    Stud78,
    Stud7nsq,
    Razz,
    Draw5,
    Draw58,
    Draw5nsq,
    Lowball,
    Lowball27,
    ShortDeck,
    NumGames,
}

// Structure defining the parameters of a specific game
pub struct GameParams {
    pub game: Game,         // Game type.
    pub minpocket: i32,     // Minimum number of hole cards.
    pub maxpocket: i32,     // Maximum number of hole cards.
    pub maxboard: i32,      // Maximum number of board cards.
    pub haslopot: i32,      // Indicates whether the game has a low pot
    pub hashipot: i32,      // Indicates whether the game has a high pot
    pub name: &'static str, // Name of the variant.
}
// Enumeration defining the sampling types for evaluation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SampleType {
    Exhaustive,      // Exhaustive evaluation of all possible hands
    Sample,          // Evaluation based on a Monte Carlo sample
    QuasiMonteCarlo, // Evaluation based on a Quasi-Monte Carlo sample (Sobol)
}
// Structure storing the evaluation results of a game
#[derive(Debug, Serialize, Deserialize)]
pub struct EnumResult {
    pub game: Game,              // Game type evaluated
    pub sample_type: SampleType, // Sampling type used
    pub nsamples: u32,           // Number of samples considered in the evaluation
    pub nplayers: u32,           // Number of players in the game
    // Statistics for high (wins, ties, losses)
    pub nwinhi: [u32; ENUM_MAXPLAYERS],
    pub ntiehi: [u32; ENUM_MAXPLAYERS],
    pub nlosehi: [u32; ENUM_MAXPLAYERS],
    // Statistics for low (wins, ties, losses)
    pub nwinlo: [u32; ENUM_MAXPLAYERS],
    pub ntielo: [u32; ENUM_MAXPLAYERS],
    pub nloselo: [u32; ENUM_MAXPLAYERS],
    pub nscoop: [u32; ENUM_MAXPLAYERS], // Number of times a player scoops the entire pot
    // Share distribution for high and low
    pub nsharehi: [[u32; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
    pub nsharelo: [[u32; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
    pub nshare: Box<[[[u32; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS]>,
    pub ev: [f64; ENUM_MAXPLAYERS], // Average equity of each player

    // Safe nullable pointer to an enumeration ordering structure
    pub ordering: Option<Box<EnumOrdering>>,
}

impl Default for EnumResult {
    fn default() -> Self {
        Self {
            game: Game::Holdem,
            sample_type: SampleType::Exhaustive,
            nsamples: 0,
            nplayers: 0,
            nwinhi: [0; ENUM_MAXPLAYERS],
            ntiehi: [0; ENUM_MAXPLAYERS],
            nlosehi: [0; ENUM_MAXPLAYERS],
            nwinlo: [0; ENUM_MAXPLAYERS],
            ntielo: [0; ENUM_MAXPLAYERS],
            nloselo: [0; ENUM_MAXPLAYERS],
            nscoop: [0; ENUM_MAXPLAYERS],
            nsharehi: [[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
            nsharelo: [[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS],
            nshare: Box::new([[[0; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS + 1]; ENUM_MAXPLAYERS]),
            ev: [0.0; ENUM_MAXPLAYERS],
            ordering: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_result_serialization() {
        let mut res = EnumResult::new(Game::Holdem);
        res.nsamples = 100;
        res.nwinhi[0] = 50;

        let serialized = serde_json::to_string(&res).unwrap();
        let deserialized: EnumResult = serde_json::from_str(&serialized).unwrap();

        assert_eq!(res.game, deserialized.game);
        assert_eq!(res.nsamples, deserialized.nsamples);
        assert_eq!(res.nwinhi[0], deserialized.nwinhi[0]);
    }
}
