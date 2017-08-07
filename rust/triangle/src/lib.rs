
pub struct Triangle([u32; 3]);

impl Triangle {
    fn new(sides: [u32; 3]) -> Triangle {
        Triangle(sides)
    }

    pub fn is_equilateral(&self) -> bool {
        self.0[0] == self.0[1] && self.0[1] == self.0[2]
    }

    pub fn is_scalene(&self) -> bool {
        let neighbor = self.0.into_iter().cycle().skip(1).take(3);
        self.0.into_iter().zip(neighbor).all(|(a, b)| a != b)
    }

    pub fn is_isosceles(&self) -> bool {
        let neighbor = self.0.into_iter().cycle().skip(1).take(3);
        self.0.into_iter().zip(neighbor).any(|(a, b)| a == b)
    }

    pub fn build(sides: [u32; 3]) -> Result<Triangle, &'static str> {
        let no_null = sides.into_iter().all(|&x| x > 0);

        // A) this is too simple
        // let no_triangle_inequality = (sides[0] + sides[1] > sides[2]) &&
        //    (sides[1] + sides[2] > sides[0]) &&
        //    (sides[2] + sides[0] > sides[1]);

        // B) boilerplate bellow, that does the same thing as above
        let shifted_once = sides.into_iter().cycle().skip(1).take(3);
        let shifted_twice = sides.into_iter().cycle().skip(2).take(3);
        let no_triangle_inequality = sides
            .into_iter()
            .zip(shifted_once)
            .map(|(a, b)| a + b)
            .zip(shifted_twice)
            .all(|(sum, &c)| sum > c);

        if no_null && no_triangle_inequality {
            Ok(Triangle::new(sides))
        } else {
            Err("sides doesn't define a triangle")
        }
    }
}
