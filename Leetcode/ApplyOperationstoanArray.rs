// https://leetcode.com/problems/apply-operations-to-an-array

struct Solution {}

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        (0..n - 1).for_each(|i| {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        });

        nums = nums.into_iter().filter(|&x| x > 0).collect::<Vec<_>>();
        nums.resize(n, 0);

        nums
    }
}
