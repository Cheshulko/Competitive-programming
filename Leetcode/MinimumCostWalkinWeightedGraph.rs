// https://leetcode.com/problems/minimum-cost-walk-in-weighted-graph

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let adj = edges.into_iter().fold(vec![vec![]; n], |mut adj, edge| {
            let v = edge[0] as usize;
            let u = edge[1] as usize;
            let w = edge[2];

            adj[v].push((u, w));
            adj[u].push((v, w));

            adj
        });

        let mut components = vec![0; n];
        let mut component_cost = vec![];
        let mut cur_component = 0;
        let mut used = vec![false; n];

        for v in 0..n {
            if used[v] {
                continue;
            }

            let mut cur_cost = i32::MAX;
            let mut q = VecDeque::<usize>::new();
            q.push_back(v);
            used[v] = true;
            components[v] = cur_component;

            while let Some(u) = q.pop_front() {
                for &(to, w) in adj[u].iter() {
                    cur_cost &= w;
                    components[to] = cur_component;

                    if !used[to] {
                        used[to] = true;
                        q.push_back(to);
                    }
                }
            }

            component_cost.push(cur_cost);
            cur_component += 1;
        }

        query
            .into_iter()
            .map(|query| {
                let v = query[0] as usize;
                let u = query[1] as usize;

                if components[v] != components[u] {
                    -1
                } else {
                    component_cost[components[v]]
                }
            })
            .collect()
    }
}
