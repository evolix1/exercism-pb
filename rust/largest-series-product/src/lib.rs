
pub fn lsp(input: &str, serie_len: usize) -> Result<u32, &'static str> {
    if serie_len > input.len() {
        return Err("input is too short");
    } else if serie_len == 0 {
        return Ok(1);
    }

    let mut invalid_char = false;
    let res = input.chars()
        .into_iter()
        .map(|c| match c.to_digit(10) {
            Some(x) => x,
            None => { invalid_char = true; 1 }
        })
        .collect::<Vec<_>>()
        .windows(serie_len)
        .map(|win| win.iter().product())
        .max()
        .ok_or("unexpected error");

    if invalid_char { Err("invalid character found") } else { res }
}
