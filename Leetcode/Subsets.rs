// https://leetcode.com/problems/subsets

struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut ans = vec![];

        for i in 0..(1 << n) {
            let mut x = vec![];
            for j in 0..n {
                if i & (1 << j) > 0 {
                    x.push(nums[j]);
                }
            }
            ans.push(x);
        }

        ans
    }
}
