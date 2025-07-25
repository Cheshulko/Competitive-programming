// https://leetcode.com/problems/maximum-unique-subarray-sum-after-deletion

struct Solution {}

impl Solution {
    pub fn max_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        if nums[nums.len() - 1] < 0 {
            return nums[nums.len() - 1];
        }

        let mut seen = [false; 2 * 100 + 2];
        let mut ans = 0;
        for num in nums.into_iter().rev() {
            let num = (num + 100) as usize;

            if !seen[num] {
                seen[num] = true;

                if num > 100 {
                    ans += (num - 100) as i32;
                }
            }
        }

        ans
    }
}
