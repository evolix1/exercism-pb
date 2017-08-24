/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    unimplemented!()
}

fn overflow_rotate_left(value: u32, shl: u32) -> Option<u32> {
    match value.checked_shl(shl) {
        Some(x) if value.leading_zeros() >= shl => Some(x),
        _ => None
    }
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    if bytes[bytes.len() - 1] & 128 != 0 {
        return Err("invalid bytes");
    }

    bytes.iter()
        .map(|x| (x & 128 == 0, x & 127))
        .scan(0u32, |value, (ended, x)| {
            *value = match overflow_rotate_left(*value, 7) {
                Some(left_x) => (x as u32) | left_x,
                None => { return Some(None); }
            };
            let res = Some(Some((ended, *value)));
            if ended { *value = 0; }
            res
        })
        .collect::<Option<Vec<(bool, u32)>>>()
        .map(|cleared| cleared.iter()
             .filter_map(|&(ended, x)| if ended { Some(x) } else { None })
             .collect::<Vec<u32>>())
        .ok_or("invalid bytes")
}
