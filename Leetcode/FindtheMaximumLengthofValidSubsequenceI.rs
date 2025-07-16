// https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-i

struct Solution {}

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let x1 = nums.iter().filter(|&x| (x & 1) == 1).count();
        let x2 = nums.iter().filter(|&x| (x & 1) == 0).count();
        let x3 = nums
            .into_iter()
            .fold((0, 42), |(ans, state), x| {
                let add = (state != (x & 1)) as usize;

                (ans + add, x & 1)
            })
            .0;

        x1.max(x2).max(x3) as i32
    }
}
