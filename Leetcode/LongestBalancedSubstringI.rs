// https://leetcode.com/problems/longest-balanced-substring-i

struct Solution;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        use std::collections::HashSet;

        let s = s
            .into_bytes()
            .into_iter()
            .map(|b| (b - b'a') as usize)
            .collect::<Vec<_>>();

        let mut ans = 0;
        for i in 0..s.len() {
            let mut cnt = [0; 26];
            for j in i..s.len() {
                cnt[s[j]] += 1;

                if cnt
                    .iter()
                    .filter_map(|&cnt| (cnt > 0).then_some(cnt))
                    .collect::<HashSet<_>>()
                    .len()
                    == 1
                {
                    ans = ans.max(j - i + 1);
                }
            }
        }

        ans as i32
    }
}
