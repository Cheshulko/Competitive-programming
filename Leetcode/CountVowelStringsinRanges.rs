// https://leetcode.com/problems/count-vowel-strings-in-ranges

use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let vowel = ['a', 'e', 'i', 'o', 'u']
            .into_iter()
            .collect::<HashSet<_>>();

        let W = words.len();

        let mut pref = vec![0; W + 1];
        for (i, word) in words.into_iter().enumerate() {
            let word = word.chars().collect::<Vec<_>>();
            pref[i + 1] = pref[i]
                + (vowel.contains(&word[0]) && vowel.contains(&word[word.len() - 1])) as i32;
        }

        queries
            .into_iter()
            .map(|query| {
                let l = query[0] as usize;
                let r = query[1] as usize + 1;

                pref[r] - pref[l]
            })
            .collect()
    }
}
