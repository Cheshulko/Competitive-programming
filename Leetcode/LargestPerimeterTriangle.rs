// https://leetcode.com/problems/largest-perimeter-triangle

struct Solution {}

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.reverse();

        let n = nums.len();
        for i in 0..n - 2 {
            let a = nums[i];
            let b = nums[i + 1];
            let c = nums[i + 2];

            if a < b + c {
                return a + b + c;
            }
        }

        0
    }
}
