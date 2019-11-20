//! # Codec
//!
//! Library for encoding and decoding [EARTH](https://www.earth.engineering) addresses
//!
//! ```rust
//! // encode base58check address
//! use address::{Address, HashType, Network, Scheme};
//! let scheme: Scheme = Scheme::Legacy;
//! let network: Network = Network::Mainnet;
//! let hash_type: HashType = HashType::Key;
//! let address: Address = Address::new(network, scheme, hash_type);
//! let raw_address: Vec<u8> = gen_vec(48);
//! println!("Hello, {:#?}!", address);
//! address.encode(raw_address);
//! ```
//!

mod address;
mod decode;
mod encode;
pub use address::{Address, HashType, Network, Scheme};

fn main() {
    let scheme: Scheme = Scheme::Legacy;
    let network: Network = Network::Mainnet;
    let hash_type: HashType = HashType::Key;

    let address: Address = Address::new(network, scheme, hash_type);

    let raw_address: Vec<u8> = gen_vec(48);

    println!("Hello, {:#?}!", address);
    address.encode(raw_address);
}

fn gen_vec(len: u8) -> Vec<u8> {
    (0..len).collect()
}
