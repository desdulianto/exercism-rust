pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if Triangle::valid_sides(sides) {
            Some(Triangle { sides })
        } else {
            None
        }
    }

    fn valid_sides(sides: [u64; 3]) -> bool {
        sides.iter().all(|x| *x > 0) && {
            let mut s = sides;
            s.sort();
            s.iter().take(2).sum::<u64>() >= s[2]
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let [a, b, c] = self.sides;
        a == b && b == c
    }

    pub fn is_scalene(&self) -> bool {
        let [a, b, c] = self.sides;
        a != b && a != c && b != c
    }

    pub fn is_isosceles(&self) -> bool {
        let [a, b, c] = self.sides;
        a == b || a == c || b == c
    }
}
