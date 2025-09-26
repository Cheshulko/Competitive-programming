// https://leetcode.com/problems/valid-triangle-number

struct Solution {}

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let n = nums.len();

        let mut ans = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                let s = nums[i] + nums[j];
                let cnt = nums[(j + 1)..].partition_point(|&num| num < s);

                ans += cnt;
            }
        }

        ans as i32
    }
}
