// https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced

struct Solution {}

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut cnt = 0;
        let mut ans = 0;
        for c in s.chars() {
            if c == '[' {
                cnt += 1;
            } else {
                if cnt == 0 {
                    ans += 1;
                    cnt += 1;
                } else {
                    cnt -= 1;
                }
            }
        }
        ans
    }
}
