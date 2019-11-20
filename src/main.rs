use codec::{Address, Network, Scheme};

fn main() {
    // Decode base58 address
    let legacy_addr: &str = "1CM18hbqJzCnM8CaxaNQHxJcnkcYbLV5Gw";
    let mut addr = Address::decode(legacy_addr).unwrap();

    // Change the base58 address to a test network earth address
    addr.network = Network::Test;
    addr.scheme = Scheme::Earth;

    // Encode earth address
    let earth_address: String = addr.encode().unwrap();

    println!("{:#?}", earth_address);
    // earthtest:qp78r5zdgr53xszxlycksftf95wcv5a8q5khw5038k
}
