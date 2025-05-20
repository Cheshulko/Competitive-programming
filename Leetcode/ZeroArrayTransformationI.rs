// https://leetcode.com/problems/zero-array-transformation-i

struct Solution {}

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let n = nums.len();

        let mut pref: Vec<i64> = vec![0; n + 1];
        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize + 1;

            pref[l] -= 1;
            pref[r] += 1;
        }

        let mut cur = 0;
        for i in 0..n {
            cur += pref[i];

            if nums[i] as i64 + cur > 0 {
                return false;
            }
        }

        true
    }
}
