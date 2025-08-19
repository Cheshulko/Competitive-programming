// https://leetcode.com/problems/number-of-zero-filled-subarrays

struct Solution {}

impl Solution {
    pub fn zero_filled_subarray(mut nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut cnt = 0;

        nums.push(42);
        for num in nums {
            if num == 0 {
                cnt += 1;
            } else {
                if cnt > 0 {
                    ans += (cnt + 1) * cnt / 2;
                }
                cnt = 0;
            }
        }

        ans
    }
}
