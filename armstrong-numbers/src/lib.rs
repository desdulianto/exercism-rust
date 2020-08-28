pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let n = s.len();
    let sum = s
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .map(|x| (x as f64).powi(n as i32) as u32)
        .sum();
    num == sum
}
