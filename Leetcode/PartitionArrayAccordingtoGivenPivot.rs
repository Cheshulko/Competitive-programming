// https://leetcode.com/problems/partition-array-according-to-given-pivot

struct Solution {}

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let n = nums.len();

        let mut ans = vec![0; n];
        let mut k = 0;

        for i in 0..n {
            if nums[i] < pivot {
                ans[k] = nums[i];
                k += 1;
            }
        }
        for i in 0..n {
            if nums[i] == pivot {
                ans[k] = nums[i];
                k += 1;
            }
        }
        for i in 0..n {
            if nums[i] > pivot {
                ans[k] = nums[i];
                k += 1;
            }
        }

        ans
    }
}
