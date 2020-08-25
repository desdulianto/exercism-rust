pub fn factors(n: u64) -> Vec<u64> {
    let mut factor = n;
    let mut divisor: u64 = 2;
    let mut res: Vec<u64> = Vec::new();
    while factor > 1 {
        if factor % divisor == 0 {
            res.push(divisor);
            factor /= divisor;
        } else {
            divisor += 1;
        }
    }
    res
}
