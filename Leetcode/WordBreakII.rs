// https://leetcode.com/problems/word-break-ii

use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let word_dict = word_dict
            .into_iter()
            .fold(HashSet::<String>::new(), |mut hm, w| {
                hm.insert(w);
                hm
            });

        let n = s.len();
        let mut dp: Vec<Vec<String>> = vec![vec![]; n + 1];
        dp[0].push(String::new());

        for i in 1..=n {
            for j in 0..i {
                if word_dict.contains(&s[j..=(i - 1)]) {
                    let mut dpj = dp[j].clone();
                    for k in dpj.iter_mut() {
                        if !k.is_empty() {
                            k.push(' ');
                        }
                        k.extend((&s[j..=(i - 1)]).chars());
                    }

                    dp[i].extend(dpj);
                }
            }
        }

        dp[n].clone()
    }
}
