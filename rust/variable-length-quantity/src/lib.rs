use std::u32;

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter()
        .flat_map(|&x| split_bytes(x, 7).iter()
                  .enumerate()
                  .rev()
                  .skip_while(|&(i, x)| i > 0 && *x == 0)
                  .map(|(i, x)| if i == 0 { *x } else { x | 128 })
                  .collect::<Vec<u8>>())
        .collect()
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    if bytes[bytes.len() - 1] & 128 != 0 {
        return Err("invalid bytes");
    }

    bytes.iter()
        .map(|x| (x & 128 == 0, x & 127))
        .scan(0u32, |value, (ended, x)| {
            match checked_overflow_shl(*value, 7) {
                Some(left_x) => {
                    let comp = (x as u32) | left_x;
                    *value = if ended { 0 } else { comp };
                    Some(Some((ended, comp)))
                },
                None => { Some(None) }
            }
        })
    .collect::<Option<Vec<(bool, u32)>>>()
        .map(|cleared| cleared.iter()
             .filter_map(|&(ended, x)| if ended { Some(x) } else { None })
             .collect::<Vec<u32>>())
        .ok_or("invalid bytes")
}

fn split_bytes(x: u32, bit_count: usize) -> Vec<u8> {
    let mask = u32::MAX ^ (u32::MAX << bit_count);
    (0..(1 + 31 / bit_count) as u32)
        .map(|i| ((x.overflowing_shr(i * bit_count as u32).0 & mask) as u8))
        .collect()
}

fn checked_overflow_shl(value: u32, shl: u32) -> Option<u32> {
    match value.checked_shl(shl) {
        Some(x) if value.leading_zeros() >= shl => Some(x),
        _ => None
    }
}

