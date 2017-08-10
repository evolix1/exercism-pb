
pub struct Brackets(&'static str);

#[derive(PartialEq)]
enum Symbol {
    Bracket,
    Brace,
    Parenthesis
}

#[derive(PartialEq)]
enum Direction {
    Opening,
    Closing
}

struct Tag(Symbol, Direction);

impl Tag {
    fn from(x: char) -> Option<Tag> {
        match x {
            '[' => Some(Tag(Symbol::Bracket, Direction::Opening)),
            ']' => Some(Tag(Symbol::Bracket, Direction::Closing)),
            '{' => Some(Tag(Symbol::Brace, Direction::Opening)),
            '}' => Some(Tag(Symbol::Brace, Direction::Closing)),
            '(' => Some(Tag(Symbol::Parenthesis, Direction::Opening)),
            ')' => Some(Tag(Symbol::Parenthesis, Direction::Closing)),
            _ => None
        }
    }
}

impl Brackets {
    pub fn from(input: &'static str) -> Brackets {
        Brackets(input)
    }

    pub fn are_balanced(&self) -> bool {
        let mut stack = Vec::new();
        for tag in self.0.chars().filter_map(|x| Tag::from(x)) {
            match tag.1 {
                Direction::Opening => stack.push(tag.0),
                Direction::Closing => {
                    match stack.pop() {
                        Some(ref symbol) if *symbol == tag.0 => (),
                        _ => return false
                    }
                }
            }
        }

        stack.is_empty()
    }
}
