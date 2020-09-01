pub struct PascalsTriangle {
    rows: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            rows: row_count,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut triangle: Vec<Vec<u32>> = Vec::new();
        for i in 1..=self.rows as usize {
            let mut t: Vec<u32> = Vec::new();
            for j in 1..=i {
                t.push(if j == 1 || j == i {
                    1
                } else {
                    triangle[i-2][j-2] + triangle[i-2][j-1]
                });
            }
            triangle.push(t);
        }
        triangle
    }
}
