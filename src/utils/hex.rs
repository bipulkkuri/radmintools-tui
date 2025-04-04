use hex::{decode, encode};

pub fn string_to_hex(input: &str) -> String {
    encode(input.as_bytes())
}

pub fn hex_to_string(hex: &str) -> String {
    let bytes = decode(hex).expect("Invalid hex string");
    String::from_utf8_lossy(&bytes).to_string()
}
