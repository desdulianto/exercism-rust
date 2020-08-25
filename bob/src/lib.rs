fn is_yelling(message: &str) -> bool {
    message
        .to_lowercase()
        .chars()
        .any(|x| ('a'..'z').contains(&x))
        && message
            .chars()
            .take_while(|x| ('a'..'z').contains(x) || ('A'..'Z').contains(x))
            .all(|x| x.is_ascii_uppercase())
}

pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.is_empty() => "Fine. Be that way!",
        m if m.ends_with("?") && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if m.ends_with("?") => "Sure.",
        m if is_yelling(m) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
