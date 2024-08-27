// https://leetcode.com/problems/path-with-maximum-probability

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Wrap(pub f64);

impl Eq for Wrap {}

impl PartialOrd for Wrap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Wrap {
    fn cmp(&self, other: &Wrap) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Solution {}

impl Solution {
    fn dijkstra(adj: &Vec<Vec<(usize, Wrap)>>, s: usize) -> Vec<Wrap> {
        let mut dist = vec![Wrap(0.0); adj.len()];
        dist[s] = Wrap(1.0);

        let mut visited = vec![false; adj.len()];
        let mut queue = BinaryHeap::new();

        queue.push((Wrap(1.0), s));
        while let Some((d, u)) = queue.pop() {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            for &(v, w) in &adj[u] {
                if dist[v].0 < d.0 * w.0 {
                    dist[v].0 = d.0 * w.0;
                    queue.push((dist[v], v));
                }
            }
        }
        dist
    }

    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        let n = n as usize;
        let start_node = start_node as usize;
        let end_node = end_node as usize;

        let mut adj = vec![vec![]; n];
        for (i, edge) in edges.into_iter().enumerate() {
            let v = edge[0] as usize;
            let u = edge[1] as usize;
            adj[v].push((u, Wrap(succ_prob[i])));
            adj[u].push((v, Wrap(succ_prob[i])));
        }

        let probs = Solution::dijkstra(&adj, start_node);

        probs[end_node].0
    }
}
