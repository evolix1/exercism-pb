
trait ScrabblePoint {
    fn scrabble_value(&self) -> u32;
}

impl ScrabblePoint for char {
    // maybe faster than hashmap lookup 
    fn scrabble_value(&self) -> u32 {
        match self {
            &'A' | &'E' | &'I' | &'O' | &'U' | &'L' | &'N' | &'R' | &'S' | &'T' => 1, 
            &'a' | &'e' | &'i' | &'o' | &'u' | &'l' | &'n' | &'r' | &'s' | &'t' => 1, 
            &'D' | &'G' => 2,
            &'d' | &'g' => 2,
            &'B' | &'C' | &'M' | &'P' => 3,
            &'b' | &'c' | &'m' | &'p' => 3,
            &'F' | &'H' | &'V' | &'W' | &'Y' => 4,
            &'f' | &'h' | &'v' | &'w' | &'y' => 4,
            &'K' | &'k' => 5,
            &'J' | &'X' => 8,
            &'j' | &'x' => 8,
            &'Q' | &'Z' => 10,
            &'q' | &'z' => 10,
            _ => 0
        }
    }
}

pub fn score(input: &str) -> u32 {
    input.chars().map(|c| c.scrabble_value()).sum()
}
