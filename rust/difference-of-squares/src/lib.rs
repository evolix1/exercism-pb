
pub fn square_of_sum(max: u32) -> u32 {
    (1..max+1).sum::<u32>().pow(2)
}

pub fn sum_of_squares(max: u32) -> u32 {
    (1..max+1).map(|v| v*v).sum()
}

pub fn difference(max: u32) -> u32 {
    square_of_sum(max) - sum_of_squares(max)
}
