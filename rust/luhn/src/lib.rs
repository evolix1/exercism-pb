fn value(i: u32, v: u32) -> u32 {
    if i % 2 == 0 {
        v
    } else if v > 9 / 2 {
        v * 2 - 9
    } else {
        v * 2
    }
}

pub fn is_valid(input: &str) -> bool {
    let mut count = 0;
    let mut total = 0;

    for item in input.chars().rev() {
        if item.is_digit(10) {
            total += value(count, item.to_string().parse::<u32>().unwrap());
            count += 1;
        } else if !item.is_whitespace() {
            // only digit and whitespace are allowed
            return false;
        }
    }

    if count <= 1 {
        return false;
    }

    total % 10 == 0
}
