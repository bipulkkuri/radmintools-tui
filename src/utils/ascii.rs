//  "Hello, World!" to "72 101 108 108 111 44 32 87 111 114 108 100 33"
pub fn string_to_ascii(input: &str) -> String {
    input
        .bytes()
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

// "72 101 108 108 111 44 32 87 111 114 108 100 33" to "Hello, World!"
pub fn ascii_to_string(input: &str) -> String {
    input
        .split_whitespace() // Split by spaces
        .filter_map(|s| s.parse::<u8>().ok()) // Parse each part as a byte (u8)
        .map(char::from) // Convert each byte into a char
        .collect() // Collect the characters into a String
}
