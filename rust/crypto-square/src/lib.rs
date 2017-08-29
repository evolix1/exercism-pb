use std::ascii::AsciiExt;

pub fn encrypt(text: &str) -> String {
    let count = text.chars()
        .filter(|x| x.is_ascii() && x.is_alphabetic())
        .count();
    let side = square_side_for_len(count);

    let mut lines = Vec::new();
    text.chars()
        .filter(|x| x.is_ascii() && x.is_alphabetic())
        .enumerate()
        .filter(|&(i, x)| {
            if i < side {
                lines.push(if i > 0 { vec![' '] } else { vec![] });
            }
            lines[i % side].push(x.to_ascii_lowercase());
            false
        })
        .next();

    lines.iter().flat_map(|x| x).collect::<String>()
}

fn square_side_for_len(length: usize) -> usize {
    (1..).filter(|x| x * x >= length).next().expect(
        "must have found a size",
    )
}
