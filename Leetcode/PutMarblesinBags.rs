// https://leetcode.com/problems/put-marbles-in-bags

struct Solution {}

impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let n = weights.len();

        let weights = weights.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut pairs = weights.windows(2).map(|w| w[0] + w[1]).collect::<Vec<_>>();

        let sum = 2 * weights.iter().sum::<i64>();

        pairs.sort_unstable();
        let ma = sum - pairs.iter().take(n - k).sum::<i64>();
        let mi = sum - pairs.iter().rev().take(n - k).sum::<i64>();

        ma - mi
    }
}
