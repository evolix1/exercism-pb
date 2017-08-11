const BOARD_SIZE: u32 = 8;

pub struct ChessPosition {
    x: u32,
    y: u32
}

impl ChessPosition {
    pub fn new(x: i32, y: i32) -> Result<ChessPosition, ()> {
        let x = try_from(x)?;
        let y = try_from(y)?;

        match (x < BOARD_SIZE, y < BOARD_SIZE) {
            (true, true) => Ok(ChessPosition { x: x, y: y }),
            _ => Err(())
        }
    }
}

pub struct Queen {
    pos: ChessPosition
}

impl Queen {
    pub fn new(p: ChessPosition) -> Queen {
        Queen { pos: p }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let horizontal = abs_dist(self.pos.x, other.pos.x);
        let vertical = abs_dist(self.pos.y, other.pos.y);
        let diagonal = abs_dist(horizontal, vertical);
        return horizontal == 0 || vertical == 0 || diagonal == 0;
    }
}

fn abs_dist(a: u32, b: u32) -> u32 {
    if a < b {
        b - a
    }
    else {
        a - b
    }
}


// simulate u32::try_from (still experimental)
fn try_from(value: i32) -> Result<u32, ()> {
    if value < 0 {
        Err(())
    }
    else {
        Ok(value as u32)
    }
}
