use crate::Base58CheckError;
use bitcoin_hashes::{sha256d::Hash as Sha256d, Hash};

// pub fn encode() {
//     let base58_chars: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

//     #[rustfmt::skip]
//     let base58_digits: [Option<u8>; 128] = [
//         None,     None,     None,     None,     None,     None,     None,     None,     // 0-7
//         None,     None,     None,     None,     None,     None,     None,     None,     // 8-15
//         None,     None,     None,     None,     None,     None,     None,     None,     // 16-23
//         None,     None,     None,     None,     None,     None,     None,     None,     // 24-31
//         None,     None,     None,     None,     None,     None,     None,     None,     // 32-39
//         None,     None,     None,     None,     None,     None,     None,     None,     // 40-47
//         None,     Some(0),  Some(1),  Some(2),  Some(3),  Some(4),  Some(5),  Some(6),  // 48-55
//         Some(7),  Some(8),  None,     None,     None,     None,     None,     None,     // 56-63
//         None,     Some(9),  Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), // 64-71
//         Some(16), None,     Some(17), Some(18), Some(19), Some(20), Some(21), None,     // 72-79
//         Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), // 80-87
//         Some(30), Some(31), Some(32), None,     None,     None,     None,     None,     // 88-95
//         None,     Some(33), Some(34), Some(35), Some(36), Some(37), Some(38), Some(39), // 96-103
//         Some(40), Some(41), Some(42), Some(43), None,     Some(44), Some(45), Some(46), // 104-111
//         Some(47), Some(48), Some(49), Some(50), Some(51), Some(52), Some(53), Some(54), // 112-119
//         Some(55), Some(56), Some(57), None,     None,     None,     None,     None,     // 120-127
//     ];

//     println!("ENCODING, {:#?}", base58_chars);
// }

pub fn from_base58_str(data: &str) -> Result<Vec<u8>, Base58CheckError> {
    let base58_chars: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

    #[rustfmt::skip]
    let base58_digits: [Option<u8>; 128] = [
        None,     None,     None,     None,     None,     None,     None,     None,     // 0-7
        None,     None,     None,     None,     None,     None,     None,     None,     // 8-15
        None,     None,     None,     None,     None,     None,     None,     None,     // 16-23
        None,     None,     None,     None,     None,     None,     None,     None,     // 24-31
        None,     None,     None,     None,     None,     None,     None,     None,     // 32-39
        None,     None,     None,     None,     None,     None,     None,     None,     // 40-47
        None,     Some(0),  Some(1),  Some(2),  Some(3),  Some(4),  Some(5),  Some(6),  // 48-55
        Some(7),  Some(8),  None,     None,     None,     None,     None,     None,     // 56-63
        None,     Some(9),  Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), // 64-71
        Some(16), None,     Some(17), Some(18), Some(19), Some(20), Some(21), None,     // 72-79
        Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), // 80-87
        Some(30), Some(31), Some(32), None,     None,     None,     None,     None,     // 88-95
        None,     Some(33), Some(34), Some(35), Some(36), Some(37), Some(38), Some(39), // 96-103
        Some(40), Some(41), Some(42), Some(43), None,     Some(44), Some(45), Some(46), // 104-111
        Some(47), Some(48), Some(49), Some(50), Some(51), Some(52), Some(53), Some(54), // 112-119
        Some(55), Some(56), Some(57), None,     None,     None,     None,     None,     // 120-127
    ];
    // 11/15 is just over log_256(58)
    let mut scratch = vec![0u8; 1 + data.len() * 11 / 15];
    // Build in base 256
    for d58 in data.bytes() {
        // Compute "X = X * 58 + next_digit" in base 256
        if d58 as usize > base58_digits.len() {
            return Err(Base58CheckError::InvalidChar(d58 as char));
        }
        let mut carry = match base58_digits[d58 as usize] {
            Some(d58) => u32::from(d58),
            None => {
                return Err(Base58CheckError::InvalidChar(d58 as char));
            }
        };
        for d256 in scratch.iter_mut().rev() {
            carry += u32::from(*d256) * 58;
            *d256 = carry as u8;
            carry /= 256;
        }
        assert_eq!(carry, 0);
    }

    // Copy leading zeroes directly
    let mut ret: Vec<u8> = data
        .bytes()
        // .take_while(|&x| x == base58_digits[0])
        .map(|_| 0)
        .collect();
    // Copy rest of string
    ret.extend(scratch.into_iter().skip_while(|&x| x == 0));
    Ok(ret)
}
