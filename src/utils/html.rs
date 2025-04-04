use html_escape::{decode_html_entities, encode_text};

pub fn encode_html_string(input: String) -> String {
    encode_text(&input).to_string()
}

pub fn decode_html_string(input: String) -> String {
    decode_html_entities(&input).to_string()
}
