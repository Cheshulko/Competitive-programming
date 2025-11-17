// https://leetcode.com/problems/check-if-all-1s-are-at-least-length-k-places-away

struct Solution {}

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let ones = nums
            .into_iter()
            .enumerate()
            .filter_map(|(i, num)| (num == 1).then_some(i))
            .collect::<Vec<_>>();

        ones.windows(2)
            .map(|w| w[1] - w[0] > k as usize)
            .all(|ok| ok)
    }
}
