use codec::{Address, Network, Scheme};

fn main() {
    // Decode base58 address
    let legacy_addr: &str = "1CM18hbqJzCnM8CaxaNQHxJcnkcYbLV5Gw";

    // Decode earth address
    let earth_address: &str = "bitcoincash:qp78r5zdgr53xszxlycksftf95wcv5a8q5szslvspl";

    let mut addr = Address::decode(legacy_addr).unwrap();
    println!("{:#?}", addr);

    // Change the base58 address to a test network earth address
    addr.network = Network::Main;
    addr.scheme = Scheme::CashAddr;

    // Encode earth address
    let earth_address: String = addr.encode().unwrap();

    println!("{:#?}", earth_address);
    // earthtest:qp78r5zdgr53xszxlycksftf95wcv5a8q5khw5038k
}
