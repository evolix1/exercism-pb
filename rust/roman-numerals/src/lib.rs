
pub struct Roman(i32);

impl Roman {
    pub fn from(x: i32) -> Roman {
        Roman(x)
    }

    fn labels_at(i: usize) -> (&'static str, &'static str, &'static str) {
        match i {
            0 => ("I", "V", "X"),
            1 => ("X", "L", "C"),
            2 => ("C", "D", "M"),
            3 => ("M", "?", "?"),
            _ => unreachable!()
        }
    }

    fn base_repr(x: i32, unit: &str, half: &str, base: &str) -> String {
        match x {
            0...3 => unit.to_string().repeat(x as usize),
            4 => unit.to_string() + &half.to_string(), 
            5...8 => half.to_string() + &unit.to_string().repeat(x as usize - 5),
            9 => unit.to_string() + &base.to_string(),
            _ => unreachable!()
        }
    }

    pub fn to_string(&self) -> String {
        let mut repr = String::new();

        let mut rest = self.0;
        let mut i = 0;
        while rest > 0 {
            let field = rest % 10;
            let marks = Self::labels_at(i);

            repr = Self::base_repr(field, marks.0, marks.1, marks.2) + &repr;

            rest = (rest - field) / 10;
            i += 1;
        }

        repr
    }
}
