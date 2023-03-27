// https://leetcode.com/problems/minimum-number-of-increments-on-subarrays-to-form-a-target-array

struct Solution {}

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut cur = 0;

        for i in 0..target.len() {
            if target[i] > cur {
                ans += target[i] - cur;
            }
            cur = target[i];
        }

        ans
    }
}
