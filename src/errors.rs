use crate::Base58Error;
use crate::CashAddrError;
use crate::EarthError;
use std::{error::Error, fmt};

/// Error concerning encoding/decoding of addresses
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AddressError {
    /// Base58 address error
    Base58(Base58Error),
    /// CashAddr error
    CashAddr(CashAddrError),
    /// EarthAddr error
    Earth(EarthError),
}

impl From<Base58Error> for AddressError {
    fn from(e: Base58Error) -> AddressError {
        AddressError::Base58(e)
    }
}

impl From<CashAddrError> for AddressError {
    fn from(e: CashAddrError) -> AddressError {
        AddressError::CashAddr(e)
    }
}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddressError::Base58(ref e) => write!(f, "base58 error: {}", e),
            AddressError::CashAddr(ref e) => write!(f, "cashaddr error: {}", e),
            AddressError::Earth(ref e) => write!(f, "earthaddr error: {}", e),
        }
    }
}

impl Error for AddressError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            AddressError::Base58(ref e) => Some(e),
            AddressError::CashAddr(ref e) => Some(e),
            AddressError::Earth(ref e) => Some(e),
        }
    }

    fn description(&self) -> &str {
        match *self {
            AddressError::Base58(_) => "base58 error",
            AddressError::CashAddr(_) => "cashaddr error",
            AddressError::Earth(_) => "earthaddr error",
        }
    }
}
