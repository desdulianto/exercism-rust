use std::collections::HashMap;

const DNA_SYMBOL: &[char] = &['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !DNA_SYMBOL.contains(&nucleotide) {
        return Err(nucleotide)
    }
    dna.chars().try_fold(0 as usize, |acc, x| {
        if !DNA_SYMBOL.contains(&x) {
            Err(x)
        } else {
            Ok(acc + if x == nucleotide { 1 } else { 0 })
        }
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    DNA_SYMBOL.iter().for_each(|x| { counts.insert(*x, 0); });
    for c in dna.chars() {
        match counts.get_mut(&c) {
            Some(count) => *count += 1,
            None => return Err(c),
        }
    }
    Ok(counts)
}
