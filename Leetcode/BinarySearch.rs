// https://leetcode.com/problems/binary-search

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();

        while r - l > 1 {
            let mid = (l + r) / 2;
            if target < nums[mid] {
                r = mid;
            } else {
                l = mid;
            }
        }

        if nums[l] == target {
            l as i32
        } else {
            -1
        }
    }
}
