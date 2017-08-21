
#[derive(Debug, PartialEq)]
pub enum Comparison {
    Unequal,
    Equal,
    Sublist,
    Superlist
}

pub fn sublist<T>(first: &[T], second: &[T]) -> Comparison where T: PartialEq {
    let (shortest, longest) =
        if first.len() < second.len() { (first, second) }
        else { (second, first) };

    let mut anchors = Vec::new();
    let mut position = 0;
    let mut done = first.len() * second.len() == 0;
    while !done && position < longest.len() {
        if let Some(start) = anchors.first().cloned() {
            if longest[position] != shortest[position - start] {
                anchors.remove(0);
                position = anchors.first().cloned().unwrap_or(position);
            }
        }

        if longest[position] == shortest[0]
            && anchors.last().map(|&last| last < position).unwrap_or(true) {
                anchors.push(position);
            }

        position += 1;

        done = anchors.first().map(|&start| position - start)
            .map(|length| length >= shortest.len()) .unwrap_or(false);
    }

    if !done { Comparison::Unequal }
    else if first.len() < second.len() { Comparison::Sublist }
    else if shortest.len() == longest.len() { Comparison::Equal }
    else { Comparison::Superlist }
}
