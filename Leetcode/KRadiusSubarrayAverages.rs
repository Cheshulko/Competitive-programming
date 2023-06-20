// https://leetcode.com/problems/k-radius-subarray-averages

struct Solution {}

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut s = nums.iter().take(2 * k).map(|x| *x as i64).sum::<i64>() + nums[0] as i64;

        vec![-1; k.min(nums.len())]
            .into_iter()
            .chain(nums.iter().enumerate().skip(2 * k).map(|(ind, cur)| {
                s -= *nums.get(ind - (2 * k + 1).min(ind)).unwrap_or(&0) as i64;
                s += *cur as i64;
                (s / (2 * k + 1) as i64) as i32
            }))
            .chain(vec![-1; k.min(nums.len())].into_iter())
            .take(nums.len())
            .collect::<Vec<_>>()
    }
}
