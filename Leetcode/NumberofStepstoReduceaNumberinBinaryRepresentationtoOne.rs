// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one

struct Solution {}

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut add = 0;
        let mut ans = 0;

        for c in s.into_bytes().into_iter().skip(1).rev() {
            if c == b'0' {
                ans += 1 + add;
            } else {
                ans += 1 + (1 - add);
                if add == 0 {
                    add = 1;
                }
            }
        }

        ans + add
    }
}
