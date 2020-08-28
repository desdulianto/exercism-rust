pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    2f64.powi((s - 1) as i32) as u64
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
