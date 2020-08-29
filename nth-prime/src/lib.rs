fn is_prime(n: u32) -> bool {
    (2..=((n as f64).sqrt().floor() as u32)).all(|i| n % i != 0)
}

pub fn nth(n: u32) -> u32 {
    match (2..).filter(|x| is_prime(*x)).nth(n as usize) {
        Some(x) => x,
        None => 0,
    }
}
