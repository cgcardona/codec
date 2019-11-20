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
use address::Address;
use decode::decode;
use encode::encode;

#[derive(Debug)]
enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

#[derive(Debug)]
enum Scheme {
    Legacy,
    CashAddr,
}

#[derive(Debug)]
enum HashType {
    Key,
    Script,
}

fn main() {
    let leg: Scheme = Scheme::Legacy;
    let a: Address = Address::new("foobar".into());

    println!("Hello, {:#?}!", a);
    decode();
    encode();
}
