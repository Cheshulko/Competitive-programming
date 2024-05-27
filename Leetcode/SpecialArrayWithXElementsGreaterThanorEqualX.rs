// https://leetcode.com/problems/special-array-with-x-elements-greater-than-or-equal-x

struct Solution {}

impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.reverse();

        for x in 0..=nums[0] {
            if nums.partition_point(|y| y >= &x) as i32 == x {
                return x;
            }
        }

        return -1;
    }
}
