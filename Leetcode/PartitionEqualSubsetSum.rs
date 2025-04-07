// https://leetcode.com/problems/partition-equal-subset-sum

struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        const MAX_DIF: usize = 100 * 200;

        let mut dp = [false; 2 * MAX_DIF + 1];
        let mut ndp = [false; 2 * MAX_DIF + 1];

        dp[MAX_DIF] = true;
        for num in nums {
            let num = num as usize;

            for i in 0..=2 * MAX_DIF {
                ndp[i] = false;
            }

            for dif in 0..=2 * MAX_DIF {
                if dif >= num {
                    ndp[dif - num] |= dp[dif];
                }
                if dif + num <= 2 * MAX_DIF {
                    ndp[dif + num] |= dp[dif];
                }
            }

            std::mem::swap(&mut dp, &mut ndp);
        }

        dp[MAX_DIF]
    }
}
