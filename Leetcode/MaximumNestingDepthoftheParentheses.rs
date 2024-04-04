// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses

struct Solution {}

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut ans = 0;
        let mut cur = 0;

        for c in s.bytes() {
            match c {
                b'(' => {
                    cur += 1;
                    ans = ans.max(cur);
                }
                b')' => cur -= 1,
                _ => {}
            }
        }

        ans
    }
}
