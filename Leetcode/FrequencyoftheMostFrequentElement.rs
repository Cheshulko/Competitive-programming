// https://leetcode.com/problems/frequency-of-the-most-frequent-element

struct Solution {}

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as i64;
        let mut nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        nums.sort_unstable();

        let mut suffix_sum = vec![(0, 0); n + 1];
        for (ind, num) in nums.iter().enumerate().rev() {
            suffix_sum[ind] = (suffix_sum[ind + 1].0 + *num, (n - ind) as i64)
        }

        let mut ans = 0;

        for (ind, num) in nums.into_iter().enumerate().rev() {
            let extra_from_right = suffix_sum[ind + 1];
            let res = suffix_sum[..=ind].binary_search_by(|suffix| {
                let sum_ = suffix.0 - extra_from_right.0;
                let ind_ = suffix.1 - extra_from_right.1;
                sum_.cmp(&(num * ind_ - k))
            });

            ans = ans.max(
                ind + 1
                    - match res {
                        Ok(ind_) => ind_,
                        Err(ind_) => ind_,
                    },
            );
        }

        ans as i32
    }
}
