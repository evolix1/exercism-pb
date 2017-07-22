use std::collections::HashMap;

pub fn word_count(input: &str) -> HashMap<String, u32> {
    let mut dict = HashMap::new();

    let last_word = input.chars()
        .fold(String::new(), |mut acc, x| {
            match x.is_alphabetic() || x.is_digit(10) {
                true => { acc.push(x); acc },
                false if !acc.is_empty() => {
                    *dict.entry(acc.to_lowercase()).or_insert(0) += 1;
                    String::new()
                },
                false => acc
            }
        });

    if !last_word.is_empty() {
        *dict.entry(last_word.to_lowercase()).or_insert(0) += 1;
    }

    dict
}
