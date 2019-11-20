//! # Codec
//!
//! Struct and impl for EARTH Codec
//!

use crate::from_base58_str;

/// Networks
#[derive(Debug)]
pub enum Network {
    /// Mainnet
    Mainnet,
    /// Testnet
    Testnet,
    /// Regtest
    Regtest,
}

/// Schemes
#[derive(Debug)]
pub enum Scheme {
    /// Base58Check
    Base58Check,
    /// CashAddr
    CashAddr,
}

/// HashTypes
#[derive(Debug)]
pub enum HashType {
    // Key
    Key,
    // Script
    Script,
}

#[derive(Debug)]
pub struct Codec {
    network: Network,
    scheme: Scheme,
    hash_type: HashType,
}

impl Codec {
    pub fn new(network: Network, scheme: Scheme, hash_type: HashType) -> Self {
        Codec {
            network: network,
            scheme: scheme,
            hash_type: hash_type,
        }
    }

    pub fn encode(&self, data: Vec<u8>) {
        println!("ENCODE: {:#?}", data);
    }
    pub fn decode(&self, data: String) {
        from_base58_str(&data);
    }
}
