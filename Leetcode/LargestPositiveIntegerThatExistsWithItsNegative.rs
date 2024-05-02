// https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative

struct Solution {}

impl Solution {
    pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let n = nums.len();
        let mut i = 0;
        let mut j = n - 1;

        while i <= j {
            if nums[i] < 0 {
                if nums[i].abs() > nums[j] {
                    i += 1;
                } else if nums[i].abs() == nums[j] {
                    return nums[j];
                } else {
                    j -= 1;
                }
            } else {
                break;
            }
        }

        return -1;
    }
}
