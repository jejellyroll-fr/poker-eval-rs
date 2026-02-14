use std::error::Error;
use std::fmt;

/// Common error type for poker-eval-rs operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PokerError {
    /// Input validation failed (e.g., duplicate cards, invalid masks).
    InvalidInput(String),
    /// The number of players exceeds the maximum allowed.
    TooManyPlayers,
    /// The specified game type is not supported.
    UnsupportedGameType,
    /// The board configuration is invalid for the game type.
    UnsupportedBoardConfiguration,
    /// An invalid card configuration was detected.
    InvalidCardConfiguration(String),
    /// Simulation or enumeration failed.
    ExecutionError(String),
    /// Internal error (should not happen in normal usage).
    InternalError(String),
    /// A generic error with a descriptive message.
    Other(String),
}

impl fmt::Display for PokerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PokerError::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
            PokerError::TooManyPlayers => write!(f, "Too many players"),
            PokerError::UnsupportedGameType => write!(f, "Unsupported game type"),
            PokerError::UnsupportedBoardConfiguration => {
                write!(f, "Unsupported board configuration")
            }
            PokerError::InvalidCardConfiguration(msg) => {
                write!(f, "Invalid card configuration: {}", msg)
            }
            PokerError::ExecutionError(msg) => write!(f, "Execution Error: {}", msg),
            PokerError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
            PokerError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl Error for PokerError {}
