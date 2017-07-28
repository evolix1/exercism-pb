pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let empty = Vec::new();

        let mut res : Vec<Vec<u32>> = Vec::new();
        for depth in 0..self.0 {
            let mut row = Vec::with_capacity(depth as usize + 1);
            let mut last_value = 0;
            for x in res.last().unwrap_or(&empty) {
                row.push(x + last_value);
                last_value = x.clone();
            }
            row.push(1);
            res.push(row);
        }

        res
    }
}
