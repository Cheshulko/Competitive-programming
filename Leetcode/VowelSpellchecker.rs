// https://leetcode.com/problems/vowel-spellchecker

struct Solution {}

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;

        let case1 =
            wordlist
                .clone()
                .into_iter()
                .enumerate()
                .fold(HashMap::new(), |mut hm, (i, word)| {
                    hm.insert(word, i);
                    hm
                });

        let case2 =
            wordlist
                .clone()
                .into_iter()
                .enumerate()
                .fold(HashMap::new(), |mut hm, (i, word)| {
                    let word = word.to_ascii_lowercase();
                    if !hm.contains_key(&word) {
                        hm.insert(word, i);
                    }

                    hm
                });

        let case3 =
            wordlist
                .clone()
                .into_iter()
                .enumerate()
                .fold(HashMap::new(), |mut hm, (i, word)| {
                    let word = word
                        .to_ascii_lowercase()
                        .into_bytes()
                        .into_iter()
                        .map(|b| {
                            if ['a', 'e', 'i', 'o', 'u'].contains(&(b as char).to_ascii_lowercase())
                            {
                                b'^'
                            } else {
                                b
                            }
                        })
                        .collect::<Vec<_>>();

                    let word = String::from_utf8(word).unwrap();
                    if !hm.contains_key(&word) {
                        hm.insert(word, i);
                    }

                    hm
                });

        queries
            .into_iter()
            .map(|word| {
                if let Some(_) = case1.get(&word) {
                    return word;
                }

                let word = word.to_ascii_lowercase();
                if let Some(&i) = case2.get(&word) {
                    return wordlist[i].clone();
                }

                let word = word
                    .into_bytes()
                    .into_iter()
                    .map(|b| {
                        if ['a', 'e', 'i', 'o', 'u'].contains(&(b as char).to_ascii_lowercase()) {
                            b'^'
                        } else {
                            b
                        }
                    })
                    .collect::<Vec<_>>();

                let word = String::from_utf8(word).unwrap();
                if let Some(&i) = case3.get(&word) {
                    return wordlist[i].clone();
                }

                return String::new();
            })
            .collect()
    }
}
