use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let n = nums.len() as i64;

        let cnt =
            nums.into_iter()
                .enumerate()
                .fold(HashMap::<usize, i64>::new(), |mut hm, (i, num)| {
                    *hm.entry(num as usize - i).or_default() += 1;
                    hm
                });

        let mut ans = n * (n - 1) / 2;
        for v in cnt.into_values() {
            ans -= v * (v - 1) / 2;
        }

        ans
    }
}
