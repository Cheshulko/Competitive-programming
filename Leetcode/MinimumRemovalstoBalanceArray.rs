// https://leetcode.com/problems/minimum-removals-to-balance-array

struct Solution;

impl Solution {
    pub fn min_removal(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as i64;
        let mut nums = nums.into_iter().map(i64::from).collect::<Vec<_>>();

        nums.sort_unstable();

        let mut ans = n;
        for i in 0..n {
            let p = nums.partition_point(|&x| x <= nums[i] * k);
            ans = ans.min(i + (n - p));
        }

        ans as i32
    }
}
