// https://leetcode.com/problems/remove-all-occurrences-of-a-substring

struct Solution {}

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let part = part.chars().collect::<Vec<_>>();

        let mut ans = vec![];
        for c in s.into_iter() {
            ans.push(c);

            while ans.len() >= part.len() && &ans[ans.len() - part.len()..] == part {
                for _ in 0..part.len() {
                    ans.pop();
                }
            }
        }

        ans.into_iter().collect()
    }
}
