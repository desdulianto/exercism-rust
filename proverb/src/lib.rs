pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        String::from("")
    } else {
        [
            list.iter()
                .zip(list.iter().skip(1))
                .map(|v| format!("For want of a {} the {} was lost.", v.0, v.1))
                .collect::<Vec<String>>(),
            vec![format!("And all for the want of a {}.", list[0])],
        ]
        .concat()
        .join("\n")
    }
}
