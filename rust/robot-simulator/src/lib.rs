pub struct Actor {
    x: i32,
    y: i32
}

impl Actor {
    fn follow(self, dir: &Direction) -> Actor {
        match *dir {
            Direction::North => Actor { x: self.x, y: self.y + 1 },
            Direction::East => Actor { x: self.x + 1, y: self.y },
            Direction::South => Actor { x: self.x, y: self.y - 1 },
            Direction::West => Actor { x: self.x - 1, y: self.y }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    // (Left turn, Right turn)
    fn turns(&self) -> (Direction, Direction) {
        match *self {
            Direction::North => (Direction::West, Direction::East),
            Direction::East => (Direction::North, Direction::South),
            Direction::South => (Direction::East, Direction::West),
            Direction::West => (Direction::South, Direction::North)
        }
    }
}

pub struct Robot(Actor, Direction);

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot(
            Actor {
                x : x as i32,
                y : y as i32 },
                d
             )
    }

    pub fn turn_right(self) -> Self {
        Robot(self.0, self.1.turns().1)
    }

    pub fn turn_left(self) -> Self {
        Robot(self.0, self.1.turns().0)
    }

    pub fn advance(self) -> Self {
        Robot(self.0.follow(&self.1), self.1)
    }

    fn obey(self, code: &str) -> Self {
        match code {
            "R" => self.turn_right(),
            "L" => self.turn_left(),
            "A" => self.advance(),
            _ => panic!("Invalid instruction code")
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        match instructions.split_at(1) {
            (c, "") => self.obey(c),
            (c, rest) => self.obey(c).instructions(rest)
        }
    }

    pub fn position(&self) -> (isize, isize) {
        (self.0.x as isize, self.0.y as isize)
    }

    pub fn direction(&self) -> &Direction {
        &self.1
    }
}
