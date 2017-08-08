
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, ()> {
    if from_base <= 1 {
        return Err(());
    }

    number.iter()
        .rev()
        .zip((0..).map(|i| from_base.pow(i as u32)))
        .map(|(x, base)| if x < &from_base { Some(base * x) } else { None })
        .collect::<Option<Vec<u32>>>()
        .and_then(|m: Vec<u32>| encode(m.iter().sum(), to_base).ok())
        .ok_or(())
}

pub fn encode(mut number: u32, to_base: u32) -> Result<Vec<u32>, ()> {
    if to_base <= 1 {
        return Err(())
    }

    let mut res = Vec::new();
    while number > 0 {
        let rest = number % to_base;
        number = (number - rest) / to_base;
        res.insert(0, rest);
    }
    Ok(res)
}

