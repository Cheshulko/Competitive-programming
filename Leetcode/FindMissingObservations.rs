// https://leetcode.com/problems/find-missing-observations

struct Solution {}

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let s = rolls.iter().sum::<i32>();
        let sa = mean * (n + rolls.len() as i32);
        let need = sa - s;

        if need < n || need > 6 * n {
            return vec![];
        }

        let mut need = need - n;
        let mut ans = vec![];
        for _ in 0..n {
            let can = need.min(5);
            ans.push(can + 1);
            need -= can;
        }

        ans
    }
}
