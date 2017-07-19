
fn has_case(c: char) -> bool {
    c.to_lowercase().ne(c.to_uppercase())
}

fn is_uppercase_if_has_case(c: char) -> bool {
    c.is_uppercase()
    || (c.to_lowercase().eq(c.to_uppercase()))
}

pub fn reply(raw_question: &'static str) -> &'static str {
    let question = raw_question.trim_left().trim_right();

    if question.chars().all(is_uppercase_if_has_case) 
        && question.chars().any(has_case) {
        "Whoa, chill out!"
    } else if question.ends_with("?") {
        "Sure."
    } else if question.is_empty() {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}
