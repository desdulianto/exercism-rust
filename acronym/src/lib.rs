fn clean_str(s: &str) -> String {
    s.chars()
        .map(|x| {
            if x.is_alphanumeric() || x == '\'' {
                x
            } else {
                ' '
            }
        })
        .collect::<String>()
}

fn split_camel_case(s: &str) -> String {
    s.char_indices()
        .map(|(i, c)| {
            if c.is_uppercase() {
                match (
                    s.chars().nth((i as isize - 1) as usize),
                    s.chars().nth(i + 1),
                ) {
                    (Some(p), Some(n)) if p.is_lowercase() && n.is_lowercase() => format!(" {}", c),
                    _ => format!("{}", c),
                }
            } else {
                format!("{}", c)
            }
        })
        .collect::<String>()
}

pub fn abbreviate(phrase: &str) -> String {
    let cleaned_str = clean_str(split_camel_case(phrase).as_str());
    let words = cleaned_str.split_whitespace().collect::<Vec<&str>>();
    words
        .iter()
        .map(|s| s.chars().next().unwrap().to_uppercase().to_string())
        .collect::<String>()
}
