// https://leetcode.com/problems/parallel-courses-iii

struct Solution {}

impl Solution {
    fn dfs(cur: usize, dp: &mut Vec<usize>, adj_rev: &Vec<Vec<usize>>, time: &Vec<i32>) -> usize {
        if dp[cur] != 0 {
            return dp[cur];
        }

        let cur_t = time[cur] as usize;
        let mut depth = cur_t;

        for to in &adj_rev[cur] {
            depth = depth.max(cur_t + Solution::dfs(*to, dp, adj_rev, time));
        }

        dp[cur] = depth;

        depth
    }

    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut outs = vec![0; n];
        let mut adj_rev = vec![vec![]; n];
        let mut dp = vec![0; n];

        for relation in relations.into_iter() {
            let from = relation[0] as usize - 1;
            let to = relation[1] as usize - 1;
            outs[from] += 1;
            adj_rev[to].push(from);
        }

        let starts = outs
            .into_iter()
            .enumerate()
            .filter(|(_, cnt)| cnt == &0)
            .map(|(ind, _)| ind)
            .collect::<Vec<_>>();

        let mut ans = 0;
        for start in starts.into_iter() {
            ans = ans.max(Solution::dfs(start, &mut dp, &adj_rev, &time));
        }

        ans as i32
    }
}
