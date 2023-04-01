use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut ans: HashSet<&'a str> = HashSet::new();

    let word_lower = word
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|x| x.to_lowercase().to_string())
        .collect::<String>()
        .chars()
        .collect::<Vec<char>>();

    let mut word_lower_sorted = word_lower.clone();
    word_lower_sorted.sort_by(|a, b| b.cmp(a));
    let word_lower_sorted = String::from_iter(word_lower_sorted);

    println!("P {}", word_lower_sorted);

    possible_anagrams.iter().for_each(|possible| {
        let mut possible_lower = possible
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|x| x.to_lowercase().to_string())
            .collect::<String>()
            .chars()
            .collect::<Vec<char>>();

        let the_same = possible_lower.cmp(&word_lower);
        possible_lower.sort_by(|a, b| b.cmp(a));
        let possible_lower = String::from_iter(possible_lower);

        if possible_lower.cmp(&word_lower_sorted).is_eq() && !the_same.is_eq() {
            ans.insert(possible);
        }
    });
    ans
}
