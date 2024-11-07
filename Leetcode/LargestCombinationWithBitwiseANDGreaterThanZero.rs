// https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero

struct Solution {}

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut ans = 0;
        for b in (0..32).rev() {
            let mut cnt = 0;
            for &x in candidates.iter() {
                if x & (1 << b) > 0 {
                    cnt += 1;
                }
            }
            ans = ans.max(cnt);
        }

        ans as i32
    }
}
