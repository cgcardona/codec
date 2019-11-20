//! # Address
//!
//! Struct and impl for EARTH Addresses
//!

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
    /// Legacy
    Legacy,
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
pub struct Address {
    network: Network,
    scheme: Scheme,
    hash_type: HashType,
}

impl Address {
    pub fn new(network: Network, scheme: Scheme, hash_type: HashType) -> Self {
        Address {
            network: network,
            scheme: scheme,
            hash_type: hash_type,
        }
    }

    pub fn encode(&self, data: Vec<u8>) {
        println!("ENCODE: {:#?}", data);
    }

    pub fn decode(&self) {
        println!("DECODE");
    }
}
