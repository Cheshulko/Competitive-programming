// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid

struct Solution {}

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut cnt = 0;
        let mut ans = 0;
        for c in s.chars() {
            if c == '(' {
                cnt += 1;
            } else {
                if cnt == 0 {
                    ans += 1;
                } else {
                    cnt -= 1;
                }
            }
        }
        ans + cnt
    }
}
