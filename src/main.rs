//! # Codec
//!
//! Library for encoding and decoding [EARTH](https://www.earth.engineering) addresses
//!
//! ```rust
//! // encode base58check address
//! use Address::*;
//!
//! ```
//!

mod address;
mod decode;
mod encode;
pub use address::{Address, HashType, Network, Scheme};
use decode::decode;
use encode::encode;

fn main() {
    let scheme: Scheme = Scheme::Legacy;
    let network: Network = Network::Mainnet;
    let hash_type: HashType = HashType::Key;

    let address: Address = Address::new(network, scheme, hash_type);

    println!("Hello, {:#?}!", address);
    decode();
    encode();
}
