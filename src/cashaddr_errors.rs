use std::{error::Error, fmt};

/// Error concerning encoding/decoding of cashaddrs
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CashAddrError {
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

impl fmt::Display for CashAddrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CashAddrError::ChecksumFailed(actual) => {
                write!(f, "invalid checksum (actual {} != 0)", actual)
            }
            CashAddrError::InvalidChar(index) => write!(f, "invalid char ({})", index),
            CashAddrError::NoPrefix => write!(f, "zero or multiple prefixes"),
            CashAddrError::MixedCase => write!(f, "mixed case string"),
            CashAddrError::InvalidVersion(c) => write!(f, "invalid version byte ({})", c),
            CashAddrError::InvalidPrefix(prefix) => write!(f, "invalid prefix ({})", prefix),
            CashAddrError::InvalidLength(length) => write!(f, "invalid length ({})", length),
        }
    }
}

impl Error for CashAddrError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
    fn description(&self) -> &str {
        match *self {
            CashAddrError::ChecksumFailed { .. } => "invalid checksum",
            CashAddrError::InvalidChar(_) => "invalid char",
            CashAddrError::NoPrefix => "zero or multiple prefixes",
            CashAddrError::MixedCase => "mixed case string",
            CashAddrError::InvalidVersion(_) => "invalid version byte",
            CashAddrError::InvalidPrefix(_) => "invalid prefix",
            CashAddrError::InvalidLength(_) => "invalid length",
        }
    }
}
