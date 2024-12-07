// https://leetcode.com/problems/minimum-limit-of-balls-in-a-bag

struct Solution {}

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        fn can(nums: &Vec<usize>, max_operations: usize, max: usize) -> bool {
            let mut ops = 0;
            for &x in nums.iter() {
                // ops + x - 1 / (ops + 1) = max
                // ops + x - 1 = (ops + 1) * max
                // x - 1 = max * ops
                ops += (x - 1) / max
            }

            return ops <= max_operations;
        }

        let nums = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let mut l = 0;
        let mut r = *nums.iter().max().unwrap();

        while r - l > 1 {
            let m = (l + r) >> 1;

            if can(&nums, max_operations as usize, m) {
                r = m;
            } else {
                l = m;
            }
        }

        return r as i32;
    }
}
