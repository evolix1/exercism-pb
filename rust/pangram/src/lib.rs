use std::ascii::AsciiExt;

pub fn is_pangram(input: &str) -> bool {
    let mut checks = (0..26).map(|x| (x + 'a' as u8) as char).collect::<Vec<char>>();
    let mut source = input.chars();

    while let Some(curr) = source.next() {
        let lower_curr = match curr.is_lowercase() {
            true => curr,
            false => curr.to_ascii_lowercase()
        };

        if let Some(i) = checks.iter().position(|&p| p == lower_curr) {
            checks.remove(i);

            if checks.is_empty() {
                return true
            }
        }

    }

    false
}
