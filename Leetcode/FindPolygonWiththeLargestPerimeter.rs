// https://leetcode.com/problems/find-polygon-with-the-largest-perimeter

struct Solution {}

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        nums.sort();
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();

        let n = nums.len();
        let mut ans = -1;
        let mut sum = nums.iter().sum::<i64>() - nums.last().unwrap();
        let mut i = n - 2;

        while i > 0 {
            if sum > nums[i + 1] {
                ans = sum + nums[i + 1];
                break;
            } else {
                sum -= nums[i];
                i -= 1;
            }
        }

        ans
    }
}
