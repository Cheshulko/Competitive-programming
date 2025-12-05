// https://leetcode.com/problems/count-partitions-with-even-sum-difference

struct Solution {}

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let right = nums.iter().sum::<i32>();

        nums.into_iter()
            .fold((0, right, 0), |(mut left, mut right, ans), num| {
                left += num;
                right -= num;

                if right == 0 {
                    return (left, right, ans);
                }

                return (left, right, ans + 1 - right.abs_diff(left) % 2);
            })
            .2 as i32
    }
}
