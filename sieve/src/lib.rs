use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut non_primes: HashSet<u64> = HashSet::new();
    (2..=upper_bound)
        .filter(|x| {
            let is_prime = !non_primes.contains(x);
            (x+x..=upper_bound).step_by(*x as usize).for_each(|y| { non_primes.insert(y); });
            is_prime
        })
        .collect()
}
