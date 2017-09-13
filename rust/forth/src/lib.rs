use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

pub struct Forth {
    stack: Vec<Value>,
    custom: HashMap<String, Vec<Token>>,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            custom: HashMap::new(),
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval<'a>(&mut self, input: &'a str) -> ForthResult {
        let mut tokens = Tokenizer::new(input);
        while let Some(token) = tokens.next() {
            let action_status = match token {
                Ok(Token::START_DEFINITION) => self.fetch_definition(&mut tokens),
                Ok(Token::LITERAL(ref name)) => self.inject_literal(&mut tokens, name),
                Ok(action) => {
                    match action.as_literal() {
                        Some(ref name) if self.custom.contains_key(name) => {
                            self.inject_literal(&mut tokens, name)
                        }
                        _ => action.execute_on(&mut self.stack),
                    }
                }
                Err(error) => Some(error),
            };

            if let Some(error) = action_status {
                return Err(error);
            }
        }

        Ok(())
    }

    fn inject_literal(&mut self, tokens: &mut Tokenizer, literal: &String) -> Option<Error> {
        let literal = literal.to_uppercase();
        match self.custom.get(&literal) {
            Some(ref queue) => {
                tokens.inject(queue);
                None
            }
            None => Some(Error::UnknownWord),
        }
    }

    fn fetch_definition(&mut self, tokens: &mut Tokenizer) -> Option<Error> {
        let name = match tokens.next() {
            Some(Ok(Token::LITERAL(name))) => name,
            Some(Ok(action)) => {
                match action.as_literal() {
                    Some(name) => name,
                    None => return Some(Error::InvalidWord),
                }
            }
            Some(Err(error)) => return Some(error),
            _ => return Some(Error::InvalidWord),
        };

        // I prefer the method "for x in xxx { done = true; break; }"
        // rather than "loop true { if xxx { break } }"
        let mut done = false; 
        let mut queue = Vec::new();
        for token in tokens {
            match token {
                Ok(Token::END_DEFINITION) => {
                    done = true;
                    break;
                }
                Ok(token) => queue.push(token),
                Err(error) => return Some(error),
            }
        }

        if done {
            self.custom.insert(name, queue);
            None
        } else {
            Some(Error::InvalidWord)
        }
    }
}

#[derive(Clone)]
enum Token {
    NUMBER(Value),
    ADD,
    SUBSTRACT,
    MULTIPLY,
    DIVIDE,
    DUP,
    DROP,
    SWAP,
    OVER,
    START_DEFINITION,
    LITERAL(String),
    END_DEFINITION,
}

impl Token {
    fn from(name: String) -> Option<Token> {
        let name = name.to_uppercase();
        match name.as_str() {
            "DUP" => Some(Token::DUP),
            "DROP" => Some(Token::DROP),
            "SWAP" => Some(Token::SWAP),
            "OVER" => Some(Token::OVER),
            "+" => Some(Token::ADD),
            "-" => Some(Token::SUBSTRACT),
            "*" => Some(Token::MULTIPLY),
            "/" => Some(Token::DIVIDE),
            ":" => Some(Token::START_DEFINITION),
            ";" => Some(Token::END_DEFINITION),
            raw => {
                match raw.parse::<Value>() {
                    Ok(number) => Some(Token::NUMBER(number)),
                    Err(_) => Some(Token::LITERAL(raw.to_owned())),
                }
            }
        }
    }

    fn as_literal(&self) -> Option<String> {
        match *self {
            Token::DUP => Some("DUP".to_owned()),
            Token::DROP => Some("DROP".to_owned()),
            Token::SWAP => Some("SWAP".to_owned()),
            Token::OVER => Some("OVER".to_owned()),
            Token::LITERAL(ref text) => Some(text.to_owned()),
            _ => None,
        }
    }

    fn single_op<F>(&self, stack: &mut Vec<Value>, func: F) -> Option<Error>
    where
        F: Fn(Value) -> Result<Vec<Value>, Error>,
    {
        if stack.is_empty() {
            Some(Error::StackUnderflow)
        } else {
            let last = stack.pop().expect("last stack value exists");
            match func(last) {
                Ok(mut x) => {
                    stack.append(&mut x);
                    None
                }
                Err(e) => Some(e),
            }
        }
    }

    fn bi_op<F>(&self, stack: &mut Vec<Value>, func: F) -> Option<Error>
    where
        F: Fn(Value, Value) -> Result<Vec<Value>, Error>,
    {
        if stack.len() < 2 {
            Some(Error::StackUnderflow)
        } else {
            let last = stack.pop().expect("last stack value exists");
            let second_last = stack.pop().expect("second last stack value exists");
            match func(second_last, last) {
                Ok(mut x) => {
                    stack.append(&mut x);
                    None
                }
                Err(e) => Some(e),
            }
        }
    }

    fn execute_on(&self, stack: &mut Vec<Value>) -> Option<Error> {
        match *self {
            Token::NUMBER(value) => {
                stack.push(value);
                None
            }
            Token::ADD => self.bi_op(stack, |second, last| Ok(vec![second + last])),
            Token::SUBSTRACT => self.bi_op(stack, |second, last| Ok(vec![second - last])),
            Token::MULTIPLY => self.bi_op(stack, |second, last| Ok(vec![second * last])),
            Token::DIVIDE => {
                self.bi_op(stack, |second, last| if last == 0 {
                    Err(Error::DivisionByZero)
                } else {
                    Ok(vec![second / last])
                })
            }
            Token::DUP => self.single_op(stack, |last| Ok(vec![last, last])),
            Token::DROP => self.single_op(stack, |_| Ok(Vec::new())),
            Token::SWAP => self.bi_op(stack, |second, last| Ok(vec![last, second])),
            Token::OVER => self.bi_op(stack, |second, last| Ok(vec![second, last, second])),
            Token::START_DEFINITION => None,
            Token::LITERAL(_) => None,
            Token::END_DEFINITION => None,
        }
    }
}

struct Tokenizer<'a> {
    source: &'a str,
    head: usize,
    extra: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    fn new<'b>(source: &'b str) -> Tokenizer<'b> {
        Tokenizer {
            source,
            head: 0,
            extra: Vec::new(),
        }
    }

    fn is_sep(element: char) -> bool {
        element.is_whitespace() || element == '\u{0000}' || element == '\u{0001}' ||
            element == '\u{1680}'
    }

    fn inject(&mut self, tokens: &Vec<Token>) {
        self.extra.reserve(tokens.len());
        for token in tokens.iter().rev() {
            self.extra.push(token.clone());
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.extra.pop() {
            return Some(Ok(token));
        }

        let seq = self.source
            .chars()
            .skip(self.head)
            .inspect(|&_| self.head += 1)
            .skip_while(|&x| Self::is_sep(x))
            .take_while(|&x| !Self::is_sep(x))
            .collect::<String>();

        if seq.is_empty() {
            None
        } else {
            Some(Token::from(seq).ok_or(Error::UnknownWord))
        }
    }
}
