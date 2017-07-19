
pub fn raindrops(count: u32) -> String {
    let mut res = String::new();

    if count % 3 == 0 {
        res.push_str("Pling");
    }

    if count % 5 == 0 {
        res.push_str("Plang");
    }

    if count % 7 == 0 {
        res.push_str("Plong");
    }

    if res.is_empty() {
        res = format!("{}", count);
    }

    res
}
