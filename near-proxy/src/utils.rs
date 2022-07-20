#[derive(Eq, PartialEq)]
enum PadDirection {
    Prefix,
    Suffix,
}

fn pad(mut buffer: Vec<u8>, direction: PadDirection) -> Vec<u8> {
    if direction == PadDirection::Prefix {
        buffer.reverse();
    }

    buffer.resize((buffer.len() + 31) / 32 * 32, 0);

    if direction == PadDirection::Prefix {
        buffer.reverse();
    }
    buffer
}

/// Custom encoder for `setNearAccountIdStatus`
pub fn abi_encode(key: String, value: String) -> Vec<u8> {
    let offset1: usize = 64;
    let offset2: usize = 32 * (3 + (key.len() + 31) / 32);
    let len1 = key.len();
    let len2 = value.len();

    return vec![
        // `cast sig "setNearAccountIdStatus(string, string)"`
        hex::decode("18bf50cd").unwrap(),
        pad(offset1.to_be_bytes().to_vec(), PadDirection::Prefix),
        pad(offset2.to_be_bytes().to_vec(), PadDirection::Prefix),
        pad(len1.to_be_bytes().to_vec(), PadDirection::Prefix),
        pad(key.as_bytes().to_vec(), PadDirection::Suffix),
        pad(len2.to_be_bytes().to_vec(), PadDirection::Prefix),
        pad(value.as_bytes().to_vec(), PadDirection::Suffix),
    ]
    .concat();
}
