use std::{error::Error, fmt};

/// Error concerning encoding/decoding of earth addresses
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EarthError {
    /// Invalid length (length)
    InvalidLength(usize),
    /// Zero or multiple prefixes
    NoPrefix,
    /// Failed to match known prefixes (prefix)
    InvalidPrefix(String),
    /// Checksum failed (checksum)
    ChecksumFailed(u64),
    /// Unexpected character (char)
    InvalidChar(char),
    /// Version byte was not recognized
    InvalidVersion(u8),
    /// Upper and lowercase address string
    MixedCase,
}

impl fmt::Display for EarthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EarthError::ChecksumFailed(actual) => {
                write!(f, "invalid checksum (actual {} != 0)", actual)
            }
            EarthError::InvalidChar(index) => write!(f, "invalid char ({})", index),
            EarthError::NoPrefix => write!(f, "zero or multiple prefixes"),
            EarthError::MixedCase => write!(f, "mixed case string"),
            EarthError::InvalidVersion(c) => write!(f, "invalid version byte ({})", c),
            EarthError::InvalidPrefix(prefix) => write!(f, "invalid prefix ({})", prefix),
            EarthError::InvalidLength(length) => write!(f, "invalid length ({})", length),
        }
    }
}

impl Error for EarthError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
    fn description(&self) -> &str {
        match *self {
            EarthError::ChecksumFailed { .. } => "invalid checksum",
            EarthError::InvalidChar(_) => "invalid char",
            EarthError::NoPrefix => "zero or multiple prefixes",
            EarthError::MixedCase => "mixed case string",
            EarthError::InvalidVersion(_) => "invalid version byte",
            EarthError::InvalidPrefix(_) => "invalid prefix",
            EarthError::InvalidLength(_) => "invalid length",
        }
    }
}
