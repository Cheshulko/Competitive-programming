// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array

struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let l = nums.partition_point(|x| x < &target);
        let r = nums.partition_point(|x| x <= &target) - 1;

        vec![
            match nums.get(l) == Some(&target) {
                true => l as i32,
                _ => -1,
            },
            match nums.get(r) == Some(&target) {
                true => r as i32,
                _ => -1,
            },
        ]
    }
}
