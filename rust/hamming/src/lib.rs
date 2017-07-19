
pub fn hamming_distance(left: &str, right: &str) -> Result<u32, ()> {
    match left.len() == right.len() {
        false => Err(()),
        true => Ok(left.chars()
            .zip(right.chars())
            .fold(0, |acc, pair| acc + if pair.0 == pair.1 { 0 } else { 1 }))
    }
}
