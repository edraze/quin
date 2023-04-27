pub fn string_to_event_buffer(string: &str) -> String {
    string.chars()
        .map(char_to_buffer)
        .collect()
}

fn char_to_buffer(char: char) -> String {
    let upper_char = char.to_uppercase();
    format!("P(Key{upper_char})R(Key{upper_char})")
}
