// https://leetcode.com/problems/clear-digits

struct Solution {}

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut ans = vec![];

        for c in s.chars().into_iter() {
            if c >= '0' && c <= '9' {
                ans.pop();
            } else {
                ans.push(c);
            }
        }

        ans.into_iter().collect()
    }
}
