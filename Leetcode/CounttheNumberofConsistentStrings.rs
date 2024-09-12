// https://leetcode.com/problems/count-the-number-of-consistent-strings

struct Solution {}

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let allowed = allowed
            .into_bytes()
            .into_iter()
            .fold(vec![false; 30], |mut v, c| {
                v[(c - b'a') as usize] = true;
                v
            });

        words
            .into_iter()
            .map(|w| w.into_bytes())
            .filter(|w| {
                w.into_iter()
                    .fold(vec![false; 30], |mut v, c| {
                        v[(*c - b'a') as usize] = true;
                        v
                    })
                    .into_iter()
                    .enumerate()
                    .all(|(i, cn)| if cn { allowed[i] } else { true })
            })
            .count() as i32
    }
}
