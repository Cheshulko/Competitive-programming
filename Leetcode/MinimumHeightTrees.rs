// https://leetcode.com/problems/minimum-height-trees

use std::collections::HashSet;

struct Solution {}

impl Solution {
    fn dfs(used: &mut Vec<bool>, adj: &Vec<Vec<usize>>, cur: usize, dep: usize) -> (usize, usize) {
        used[cur] = true;

        let mut max_dep = dep;
        let mut max_dep_v = cur;

        for to in &adj[cur] {
            if !used[*to] {
                let (to_dep, v) = Solution::dfs(used, adj, *to, dep + 1);

                if to_dep > max_dep {
                    max_dep = to_dep;
                    max_dep_v = v;
                }
            }
        }

        (max_dep, max_dep_v)
    }

    fn dfs_ans(
        used: &mut Vec<bool>,
        ans: &mut HashSet<i32>,
        adj: &Vec<Vec<usize>>,
        cur: usize,
        dep: usize,
        target_dep_1: usize,
        target_dep_2: usize,
    ) {
        used[cur] = true;

        if dep == target_dep_1 || dep == target_dep_2 {
            ans.insert(cur as i32);
        }

        for to in &adj[cur] {
            if !used[*to] {
                Solution::dfs_ans(used, ans, adj, *to, dep + 1, target_dep_1, target_dep_2);
            }
        }
    }

    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let adj = edges.into_iter().fold(vec![vec![]; n], |mut e, x| {
            let v = x[0] as usize;
            let u = x[1] as usize;
            e[v].push(u);
            e[u].push(v);
            e
        });

        let mut used = vec![false; n];
        let (_, v1) = Solution::dfs(&mut used, &adj, 0, 0);

        let mut used = vec![false; n];
        let (dep_v, v2) = Solution::dfs(&mut used, &adj, v1, 0);

        let mut used = vec![false; n];
        let mut ans1 = HashSet::<i32>::new();
        Solution::dfs_ans(
            &mut used,
            &mut ans1,
            &adj,
            v1,
            0,
            dep_v / 2,
            dep_v / 2 + dep_v % 2,
        );

        let mut used = vec![false; n];
        let mut ans2 = HashSet::<i32>::new();
        Solution::dfs_ans(
            &mut used,
            &mut ans2,
            &adj,
            v2,
            0,
            dep_v / 2,
            dep_v / 2 + dep_v % 2,
        );

        ans1.intersection(&ans2)
            .into_iter()
            .map(|x| *x)
            .collect::<Vec<_>>()
    }
}
