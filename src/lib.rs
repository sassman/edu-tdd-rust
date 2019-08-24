pub fn alphabet_position(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter(|x| x.is_alphabetic())
        .map(|x| -> u8 { x as u8 - 'a' as u8 + 1 })
        .map(|x| -> String { x.to_string() })
        .collect::<Vec<String>>()
        .join(" ")
}
