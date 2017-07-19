
pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    (2..s+1).fold(1, |acc, _| acc * 2)
}

pub fn total() -> u64 {
    let no_overflow = (1..64).fold((1, 0), |acc, _| (acc.0 * 2, acc.0 + acc.1));
    no_overflow.0 + no_overflow.1
}
