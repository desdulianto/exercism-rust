pub fn abbreviate(phrase: &str) -> String {
    let cleaned_str = phrase.replace(|c: char| !(c.is_alphanumeric() || c == '\''), " ");
    let words = cleaned_str.split_whitespace().collect::<Vec<&str>>();
    words
        .iter()
        .map(|s| {
            s.chars()
                .take(1)
                .chain(
                    // camel case
                    s.chars()
                        .skip_while(|c| c.is_uppercase()) // skip adjacent upper case
                        .filter(|c| c.is_uppercase()), // filter all uppercase
                )
                .collect::<String>()
        })
        .collect::<String>()
        .to_uppercase()
}
