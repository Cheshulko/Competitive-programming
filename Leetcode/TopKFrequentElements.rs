// https://leetcode.com/problems/top-k-frequent-elements

struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mx = 10_000_00;
        let mut v = nums.into_iter().fold(
            (0..2 * mx + 1).map(|x| (0, x)).collect::<Vec<_>>(),
            |mut v, x| {
                v[(x + mx) as usize].0 += 1;
                v
            },
        );
        v.sort_by(|a, b| a.0.cmp(&b.0).reverse());
        v.into_iter()
            .take(k as usize)
            .map(|(_, x)| (x as i32) - mx)
            .collect()
    }
}
