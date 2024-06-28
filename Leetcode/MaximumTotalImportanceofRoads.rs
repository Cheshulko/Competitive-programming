// https://leetcode.com/problems/maximum-total-importance-of-roads

struct Solution {}

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut cnt = vec![0; n];

        for road in roads.into_iter() {
            cnt[road[0] as usize] += 1;
            cnt[road[1] as usize] += 1;
        }

        cnt.sort_unstable();

        let ans = cnt
            .into_iter()
            .enumerate()
            .map(|(i, x)| (i + 1) as i64 * x)
            .sum::<i64>();

        ans
    }
}
