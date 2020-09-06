pub struct Triangle {
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if Triangle::valid_sides(sides) {
            Some(Triangle { a: sides[0], b: sides[1], c: sides[2] })
        } else {
            None
        }
    }

    fn valid_sides(sides: [u64; 3]) -> bool {
        sides.iter().all(|x| *x > 0) && {
            let mut s = [sides[0], sides[1], sides[2]];
            s.sort();
            s.iter().take(2).sum::<u64>() >= s[2]
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.a != self.c && self.b != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.a == self.c || self.b == self.c
    }
}
