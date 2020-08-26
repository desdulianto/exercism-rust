use std::collections::HashSet;
use std::iter::FromIterator;

fn is_anagram(sorted_chars: &Vec<char>, other: &str) -> bool {
    let mut other: Vec<char> = other.chars().collect();
    other.sort();
    *sorted_chars == other
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let mut sorted_chars = word.chars().collect::<Vec<char>>();
    sorted_chars.sort();

    HashSet::from_iter(
        possible_anagrams
            .iter()
            .filter(|x| {
                let other = x.to_lowercase();
                let other = other.as_str();
                word != other && is_anagram(&sorted_chars, other)
            })
            .cloned(),
    )
}
