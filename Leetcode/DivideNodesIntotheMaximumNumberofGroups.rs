// https://leetcode.com/problems/divide-nodes-into-the-maximum-number-of-groups

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for v in edges.iter() {
            let (v, u) = (v[0] as usize - 1, v[1] as usize - 1);
            adj[v].push(u);
            adj[u].push(v);
        }

        fn is_bipartite(adj: &Vec<Vec<usize>>) -> Option<Vec<i32>> {
            let n = adj.len();
            let mut sides = vec![-1; n];
            let mut is_ok = true;

            let mut q = VecDeque::<usize>::new();

            for i in 0..n {
                if sides[i] == -1 {
                    q.push_back(i);
                    sides[i] = 0;

                    while let Some(v) = q.pop_front() {
                        for &to in adj[v].iter() {
                            if sides[to] == -1 {
                                sides[to] = sides[v] ^ 1;
                                q.push_back(to);
                            } else {
                                is_ok &= sides[to] != sides[v];
                            }
                        }
                    }
                }
            }

            is_ok.then_some(sides)
        }

        if is_bipartite(&adj).is_none() {
            return -1;
        }

        fn diameter_unweighted(from: usize, adj: &Vec<Vec<usize>>, used: &mut Vec<bool>) -> usize {
            let n = adj.len();

            let mut vertices_in_component = vec![];
            {
                let mut seen = vec![false; n];
                let mut q = VecDeque::<usize>::new();

                q.push_back(from);
                seen[from] = true;
                used[from] = true;

                while let Some(cur_i) = q.pop_front() {
                    vertices_in_component.push(cur_i);
                    for &to in adj[cur_i].iter() {
                        if !seen[to] {
                            seen[to] = true;
                            used[to] = true;

                            q.push_back(to);
                        }
                    }
                }
            }

            let mut ma_dist = 0;
            for cur_i in vertices_in_component.into_iter() {
                let mut seen = vec![false; n];
                let mut q = VecDeque::<(usize, usize)>::new();

                q.push_back((cur_i, 0));
                seen[cur_i] = true;

                while let Some((cur_i, cur_dist)) = q.pop_front() {
                    ma_dist = ma_dist.max(cur_dist);

                    for &to in adj[cur_i].iter() {
                        if !seen[to] {
                            seen[to] = true;
                            q.push_back((to, cur_dist + 1));
                        }
                    }
                }
            }

            1 + ma_dist
        }

        let mut used = vec![false; n];
        let mut ans = 0;
        for i in 0..n {
            if !used[i] {
                ans += diameter_unweighted(i, &adj, &mut used) as i32;
            }
        }

        ans
    }
}
