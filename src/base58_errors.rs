use std::{error::Error, fmt};

/// Error concerning encoding/decoding of base58 addresses
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Base58Error {
    /// Unexpected character (char)
    InvalidChar(char),
    /// Checksum failed (expected, actual)
    ChecksumFailed { expected: Vec<u8>, actual: Vec<u8> },
    /// Invalid length (length)
    InvalidLength(usize),
    /// Version byte was not recognized
    InvalidVersion(u8),
}

impl fmt::Display for Base58Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Base58Error::InvalidChar(b) => write!(f, "invalid char ({})", b),
            Base58Error::ChecksumFailed { expected, actual } => write!(
                f,
                "invalid checksum (actual {:?} does not match expected {:?})",
                actual, expected
            ),
            Base58Error::InvalidLength(length) => write!(f, "invalid length ({})", length),
            Base58Error::InvalidVersion(v) => write!(f, "invalid version byte ({})", v),
        }
    }
}

impl Error for Base58Error {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
    fn description(&self) -> &str {
        match *self {
            Base58Error::InvalidChar(_) => "invalid char",
            Base58Error::ChecksumFailed { .. } => "invalid checksum",
            Base58Error::InvalidLength(_) => "invalid length",
            Base58Error::InvalidVersion(_) => "invalid version",
        }
    }
}
