// https://leetcode.com/problems/arithmetic-subarrays

struct Solution {}

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        l.into_iter()
            .zip(r.into_iter())
            .map(|(s, e)| {
                let s = s as usize;
                let e = e as usize;

                let mut arr = (s..=e).map(|i| nums[i]).collect::<Vec<_>>();
                arr.sort_unstable();

                arr.windows(3).all(|x| x[0] - x[1] == x[1] - x[2])
            })
            .collect()
    }
}
