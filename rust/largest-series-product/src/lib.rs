
mod iter_map_window;
use iter_map_window::IterMapWindow;

pub fn lsp(input: &str, serie_len: usize) -> Result<u32, &'static str> {
    if serie_len > input.len() {
        return Err("input is too short");
    } else if serie_len == 0 {
        return Ok(1);
    }

    let mut invalid_char :bool = false;

    let val = input.chars()
        .into_iter()
        .map_win(serie_len, |trunk| {
            trunk.iter()
                .map(|c| {
                    match (*c).to_digit(10) {
                        Some(x) => x,
                        None => { invalid_char = true; 1 }
                    }
                })
                .fold(1, |acc, x| acc * x)
        })
        .max()
        .expect("impossible");

    match invalid_char {
        true => Err("invalid digit"),
        false => Ok(val)
    }
}
