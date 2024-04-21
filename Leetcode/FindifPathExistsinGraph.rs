// https://leetcode.com/problems/find-if-path-exists-in-graph

use std::collections::VecDeque;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let source = source as usize;
        let destination = destination as usize;
        let n = n as usize;

        let mut q = VecDeque::<usize>::new();
        let mut used = vec![false; n];

        let mut adj = vec![vec![]; n];
        for y in edges.into_iter() {
            adj[y[0] as usize].push(y[1] as usize);
            adj[y[1] as usize].push(y[0] as usize);
        }

        q.push_back(source);
        used[source] = true;

        while let Some(cur) = q.pop_front() {
            if cur == destination {
                return true;
            }

            for to in &adj[cur] {
                let to = *to as usize;

                if !used[to] {
                    used[to] = true;
                    q.push_back(to);
                }
            }
        }

        return false;
    }
}
