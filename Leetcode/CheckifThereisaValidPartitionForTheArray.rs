// https://leetcode.com/problems/check-if-there-is-a-valid-partition-for-the-array

struct Solution {}

impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut ok = vec![false; nums.len() + 1];
        ok[0] = true;

        for i in 0..n {
            if i + 1 < n && nums[i] == nums[i + 1] {
                ok[i + 1 + 1] |= ok[i];
            }
            if i + 2 < n && nums[i] == nums[i + 1] && nums[i] == nums[i + 2] {
                ok[i + 2 + 1] |= ok[i];
            }
            if i + 2 < n && nums[i] == nums[i + 1] - 1 && nums[i] == nums[i + 2] - 2 {
                ok[i + 2 + 1] |= ok[i];
            }
        }

        ok[n]
    }
}
