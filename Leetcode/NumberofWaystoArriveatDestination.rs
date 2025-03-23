// https://leetcode.com/problems/number-of-ways-to-arrive-at-destination

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        fn dijkstra(adj: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<usize> {
            const MOD: usize = 1_000_000_000 + 7;
            let n = adj.len();

            let mut dist = vec![usize::MAX; n];
            dist[s] = 0;

            let mut dp = vec![0; n];
            dp[s] = 1;

            let mut visited = vec![false; n];
            let mut queue = BinaryHeap::new();

            queue.push(Reverse((0, s)));
            while let Some(Reverse((d, u))) = queue.pop() {
                if visited[u] {
                    continue;
                }
                visited[u] = true;
                for &(v, w) in &adj[u] {
                    if dist[v] > d + w {
                        dist[v] = d + w;
                        dp[v] = dp[u];
                        queue.push(Reverse((dist[v], v)));
                    } else if dist[v] == d + w {
                        dp[v] += dp[u];
                        dp[v] %= MOD;
                    }
                }
            }
            dp
        }

        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for road in roads.into_iter() {
            let v = road[0] as usize;
            let u = road[1] as usize;
            let w = road[2] as usize;

            adj[v].push((u, w));
            adj[u].push((v, w));
        }

        dijkstra(&adj, 0)[n - 1] as i32
    }
}
