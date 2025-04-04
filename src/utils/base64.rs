use base64::{decode_config, encode_config, URL_SAFE};

pub fn base64_encode(input: &[u8]) -> String {
    encode_config(input, URL_SAFE)
}

pub fn base64_decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    decode_config(input, URL_SAFE)
}

pub fn base64_encode_std(input: &[u8]) -> String {
    base64::encode(input)
}

pub fn base64_decode_std(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode(input)
}
