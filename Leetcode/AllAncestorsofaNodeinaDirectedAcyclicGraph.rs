// https://leetcode.com/problems/all-ancestors-of-a-node-in-a-directed-acyclic-graph

struct Solution {}

impl Solution {
    fn dfs(
        cur: usize,
        target: usize,
        adj: &Vec<Vec<usize>>,
        used: &mut Vec<bool>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        used[cur] = true;

        if cur != target {
            ans[target].push(cur as i32);
        }

        for &to in adj[cur].iter() {
            if !used[to] {
                Solution::dfs(to, target, adj, used, ans);
            }
        }
    }

    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        let mut ins = vec![0; n];

        for edge in edges.into_iter() {
            adj[edge[1] as usize].push(edge[0] as usize);
            ins[edge[1] as usize] += 1;
        }

        let mut ans = vec![vec![]; n];
        let mut used = vec![false; n];
        for i in 0..n {
            used.fill(false);

            Solution::dfs(i, i, &adj, &mut used, &mut ans);
            ans[i].sort_unstable();
        }

        ans
    }
}
