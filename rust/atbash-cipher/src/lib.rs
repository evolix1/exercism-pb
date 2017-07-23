use std::ascii::AsciiExt;


fn convert_one(c: char) -> Option<char> {
    let c = c.to_ascii_lowercase();
    if !c.is_ascii() || !c.is_alphanumeric() {
        None
    } else if c >= 'a' && c <= 'z' {
        Some((97 + 25 - (c as u8 - 97)) as char)
    } else {
        Some(c)
    }
}

pub fn encode(input: &str) -> String {
    let parts = input.chars() .filter_map(|c| convert_one(c)).collect::<Vec<char>>();
    let mut parts = parts.as_slice().chunks(5);

    let mut res = String::with_capacity(input.len() + input.len() / 5);
    if let Some(part) = parts.next() {
        res.push_str(part.into_iter().collect::<String>().as_str());
    }
    for part in parts {
        res.push_str(" ");
        res.push_str(part.into_iter().collect::<String>().as_str());
    }
    res
}

pub fn decode(input: &str) -> String {
    input.chars()
        .filter_map(|c| convert_one(c))
        .collect::<String>()
}
