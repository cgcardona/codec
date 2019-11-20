//! # Codec
//!
//! Library for encoding and decoding [EARTH](https://www.earth.engineering) addresses
//!
//! ```rust
//! use address::{Address, HashType, Network, Scheme};
//! let scheme: Scheme = Scheme::Legacy;
//! let network: Network = Network::Mainnet;
//! let hash_type: HashType = HashType::Key;
//! let address: Address = Address::new(network, scheme, hash_type);
//! let raw_address: Vec<u8> = gen_vec(48);
//!
//! println!("Hello, {:#?}!", address);
//! // encode base58check address
//! address.encode(raw_address);
//! ```
//!

mod codec;
mod decode;
mod encode;
mod errors;
pub use codec::{Codec, HashType, Network, Scheme};
pub use encode::from_base58_str;
pub use errors::Base58CheckError;

fn main() {
    let scheme: Scheme = Scheme::Base58Check;
    let network: Network = Network::Mainnet;
    let hash_type: HashType = HashType::Key;

    let codec: Codec = Codec::new(network, scheme, hash_type);

    let raw_address: Vec<u8> = gen_vec(16);

    // println!("Hello, {:#?}!", codec);
    codec.encode(raw_address);
    let base_58_check_addr: String = "14H8Wuy3EGCjfoWiAXviyx921juwxyU1vt".into();
    let result = codec.decode(base_58_check_addr);
}

fn gen_vec(len: u8) -> Vec<u8> {
    (0..len).collect()
}
