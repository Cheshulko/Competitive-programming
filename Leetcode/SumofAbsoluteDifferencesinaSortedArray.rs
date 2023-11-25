// https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array

struct Solution {}

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        let mut pref_sum = vec![0; nums.len()];
        let mut suff_sum = vec![0; nums.len()];

        for i in 0..n {
            pref_sum[i as usize] = pref_sum.get((i - 1) as usize).unwrap_or(&0) + nums[i as usize];
        }

        for i in (0..n).rev() {
            suff_sum[i as usize] = suff_sum.get((i + 1) as usize).unwrap_or(&0) + nums[i as usize];
        }

        nums.into_iter()
            .enumerate()
            .map(|(ind, x)| {
                let ind = ind as i32;

                let left_x = ind * x;
                let right_x = (n - ind - 1) * x;

                let left = pref_sum.get((ind - 1) as usize).unwrap_or(&0);
                let right = suff_sum.get((ind + 1) as usize).unwrap_or(&0);

                left_x - left + right - right_x
            })
            .collect::<Vec<_>>()
    }
}
