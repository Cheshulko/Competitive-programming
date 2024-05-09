// https://leetcode.com/problems/maximize-happiness-of-selected-children

struct Solution {}

impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;

        happiness.sort_unstable();
        happiness.reverse();
        happiness
            .into_iter()
            .enumerate()
            .take(k)
            .map(|(i, x)| ((x - i as i32) as i64).max(0))
            .sum()
    }
}
