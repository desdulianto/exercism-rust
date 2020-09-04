pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    (2..=upper_bound)
        .filter(|x| {
            if primes.iter().any(|y| x % y == 0) {
                false
            } else {
                primes.push(*x);
                true
            }
        })
        .collect()
}
