
pub fn encode(input: &str) -> String {
    let mut res = String::new();

    let mut source = input.chars();
    let mut curr = source.next();
    let mut count = 1;

    while curr != None {
        let item = source.next();

        if item == curr {
            count += 1;
        } else {
            match count {
                1 => res += curr.unwrap().to_string().as_str(),
                _ => res += format!("{}{}", count, curr.unwrap().to_string()).as_str()
            }
            count = 1;
            curr = item;
        }
    }

    res
}

pub fn decode(input: &str) -> String {
    let mut res = String::new();
    
    let mut source = input.chars();
    let mut curr = source.next();

    while curr != None {
        let mut count = String::new();
        
        while curr.unwrap().is_digit(10) {
            count += curr.unwrap().to_string().as_str();
            curr = source.next();
        }

        let key = curr.unwrap().to_string();

        res += key.repeat(count.parse::<usize>().unwrap_or(1)).as_str();

        curr = source.next();
    }

    res
}
