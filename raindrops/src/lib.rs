pub fn raindrops(n: u32) -> String {
    let rain = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let mut s = Vec::new();
    for r in rain.iter() {
        if n % r.0 == 0 {
            s.push(r.1);
        }
    }
    if !s.is_empty() {
        s.join("")
    } else {
        n.to_string()
    }
}
