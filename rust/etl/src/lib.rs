use std::collections::BTreeMap;
use std::ascii::AsciiExt;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut res = BTreeMap::new();
    for (score, values) in input {
        for item in values {
            res.insert(item.to_ascii_lowercase(), *score);
        }
    }
    res
}
