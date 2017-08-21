
#[derive(Debug, PartialEq)]
pub enum Comparison {
    Unequal,
    Equal,
    Sublist,
    Superlist
}

pub fn sublist<T>(first: &[T], second: &[T]) -> Comparison where T: PartialEq {
    // handle null cases with empty arrays
    let (shortest, longest) =
        match (first.len(), second.len()) {
            (0, 0) => return Comparison::Equal,
            (0, _) => return Comparison::Sublist,
            (_, 0) => return Comparison::Superlist,
            (x, y) if x < y => (first, second),
            _ => (second, first)
        };

    // lookup strategy:
    // + search into longuest array and compare it to the shortest
    // + compare ends when whatever sequence in the longuest is exactly the shortest
    // + it saves anchor position from the longuest when the current position
    //   matches the first item of the shortest
    // + this allows better recovering from simply restart from last matching position
    let mut anchors = Vec::new();
    let mut position = 0;
    let mut done = false;
    while !done && position < longest.len() {
        // first part: current anchor still matches or it discards it
        if let Some(start) = anchors.first().cloned() {
            if longest[position] != shortest[position - start] {
                anchors.remove(0);
                position = anchors.first().cloned().unwrap_or(position);
            }
        }

        // save anchor for later, in case
        if longest[position] == shortest[0]
            && anchors.last().map(|&last| last < position).unwrap_or(true) {
                anchors.push(position);
            }

        position += 1;

        done = anchors.first()
            .map(|&start| position - start)
            .map(|length| length >= shortest.len())
            .unwrap_or(false);
    }

    if !done {
        Comparison::Unequal
    } else if first.len() < second.len() {
        Comparison::Sublist
    } else if shortest.len() == longest.len() {
        Comparison::Equal
    } else {
        Comparison::Superlist
    }
}
