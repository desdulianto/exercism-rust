fn is_prime(n: u32) -> bool {
    for i in 2..=((n as f64).sqrt().floor() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut count: i32 = -1;
    let mut number = 2;
    loop {
        if is_prime(number) {
            count += 1;
        }
        if count as u32 == n {
            return number;
        }
        number += 1;
    }
}
