
fn swap(c: char, delta: u32) -> char {
    if c >= 'a' && c <= 'z' {
        (97 + (c as u8 - 97 + delta as u8) % 26) as char
    } else if c >= 'A' && c <= 'Z' {
        (65 + (c as u8 - 65 + delta as u8) % 26) as char
    } else {
        c
    }
}

pub fn rotate(input: &str, delta: u32) -> String {
    input.chars()
        .map(|c| swap(c, delta))
        .collect::<String>()
}
