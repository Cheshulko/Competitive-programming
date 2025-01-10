// https://leetcode.com/problems/word-subsets

struct Solution {}

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let count_chars = |word: &[u8]| {
            word.iter().fold(vec![0; 26], |mut count, c| {
                let c = (c - b'a') as usize;
                count[c] += 1;

                count
            })
        };

        let count_chars_words2 = words2
            .into_iter()
            .map(|word| count_chars(word.as_bytes()))
            .fold(vec![0; 26], |state, count| {
                count
                    .into_iter()
                    .zip(state.into_iter())
                    .map(|(c1, c2)| c1.max(c2))
                    .collect()
            });

        words1
            .into_iter()
            .filter(|word| {
                count_chars(word.as_bytes())
                    .into_iter()
                    .zip(count_chars_words2.iter())
                    .all(|(c1, &c2)| c1 >= c2)
            })
            .collect()
    }
}
