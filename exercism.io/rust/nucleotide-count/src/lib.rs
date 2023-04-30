use std::collections::HashMap;

const VALID: [char; 4] = ['A', 'C', 'G', 'T'];

fn get_invalid(dna: &str) -> Option<char> {
    let invalid = dna
        .chars()
        .filter(|n| !VALID.contains(n))
        .collect::<String>();

    if invalid.len() > 0 {
        Some(invalid.chars().next().unwrap())
    } else {
        None
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALID.contains(&nucleotide) {
        return Err(nucleotide);
    }
    if let Some(invalid) = get_invalid(dna) {
        Err(invalid)
    } else {
        Ok(dna.chars().filter(|c| c == &nucleotide).count())
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    if let Some(invalid) = get_invalid(dna) {
        Err(invalid)
    } else {
        Ok(dna.chars().fold(
            HashMap::<char, usize>::from_iter(VALID.iter().map(|v| (*v, 0))),
            |mut map, cur| {
                *map.entry(cur).or_insert(0) += 1;
                map
            },
        ))
    }
}
