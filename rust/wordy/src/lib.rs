
#[derive(Clone)]
struct Op(&'static str, &'static Fn(i32, i32) -> i32);

const OPERATIONS: [Op; 4] = [
    Op("plus", &|a, b| a + b),
    Op("minus", &|a, b| a - b),
    Op("multiplied by", &|a, b| a * b),
    Op("divided by", &|a, b| a / b)
];

pub struct WordProblem(&'static str);

impl WordProblem {
    pub fn new(sentence: &'static str) -> WordProblem {
        WordProblem(sentence)
    }

    pub fn answer(&self) -> Result<i32, ()> {
        let text = Self::cut_head(self.0)?;
        let (mut value, mut rest) = Self::read_number(text)?;

        while let Some((operation, text)) = Self::read_operation(rest) {
            let (operand, text) = Self::read_number(text)?;
            value = operation.1(value, operand) as i32;
            rest = text;
        }

        match rest {
            x if x == "?" => Ok(value),
            _ => Err(())
        }
    }

    fn read_operation(s: &'static str) -> Option<(Op, &'static str)> {
        OPERATIONS
            .into_iter()
            .filter(|&op| s.starts_with(op.0))
            .next()
            .map(|op| (op.clone(), s.split_at(op.0.len()).1.trim_left()))
    }

    fn read_number(s: &'static str) -> Result<(i32, &'static str), ()> {
        let repr = s.chars()
            .enumerate()
            .take_while(|&(i, x)| x.is_digit(10) || (i == 0 && x == '-'))
            .map(|(_, x)| x)
            .collect::<String>();
        repr.parse::<i32>()
            .map(|x| (x, s.split_at(repr.len()).1.trim_left()))
            .map_err(|_| ())
    }

    fn cut_head(s: &'static str) -> Result<&'static str, ()> {
        let expected = "What is ";
        match s.split_at(expected.len()) {
            (head, tail) if head == expected => Ok(tail),
            _ => Err(()),
        }
    }
}
