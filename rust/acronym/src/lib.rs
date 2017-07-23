use std::ascii::AsciiExt;

#[inline]
fn is_uppercase_strict(c: char) -> bool {
    c.to_ascii_lowercase() != c
}

pub fn abbreviate(input: &str) -> String {
    // short hand for 'is_ascii_uppercase_strict' : 'c.to_ascii_lowercase() != c'
    input.split(|c: char| !c.is_alphabetic())
        .filter(|s| !s.is_empty())
        .flat_map(|s| {
            s.chars()
                .fold((Vec::new(), true), |(mut res, can_capture), c| {
                    // we forbid capture for any following uppercase char if
                    // immediatly after a previous uppercase char
                    let can_capture_next_time =
                        if res.is_empty() {
                            if is_uppercase_strict(c) { res.push(c); false }
                            else { res.push(c.to_ascii_uppercase()); true }
                        }
                        else if is_uppercase_strict(c) && can_capture {
                            res.push(c);
                            false
                        }
                        else { !is_uppercase_strict(c) };

                    (res, can_capture_next_time)
                }).0
        })
    .collect::<String>()
}
